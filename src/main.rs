use libp2p::PeerId;
use libp2p::identity::{PublicKey, KeyType};

/// Converts a public key string to an enode URL format
///
/// # Arguments
/// * `public_key_hex` - Hex-encoded public key (with or without 0x prefix)
/// * `ip` - IP address for the enode URL
/// * `port` - Port number for the enode URL
///
/// # Returns
/// * `Result<String, Box<dyn std::error::Error>>` - The enode URL or an error
fn public_key_to_enode(
    public_key_hex: &str,
    ip: &str,
    port: &str
) -> Result<String, Box<dyn std::error::Error>> {
    // Remove 0x prefix if present
    let clean_hex = public_key_hex.strip_prefix("0x").unwrap_or(public_key_hex);

    // Decode the hex string to bytes
    let public_key_bytes = hex::decode(clean_hex)
        .map_err(|e| format!("Failed to decode hex string: {}", e))?;

    // Parse as a PublicKey
    let public_key = PublicKey::try_decode_protobuf(&public_key_bytes)
        .map_err(|e| format!("Failed to decode public key: {}", e))?;

    // Get uncompressed format based on key type
    let uncompressed_key = match public_key.key_type() {
        KeyType::Secp256k1 => {
            // For secp256k1, get the uncompressed format (skip the 0x04 prefix)
            let uncompressed = public_key.clone().try_into_secp256k1()?.to_bytes_uncompressed();
            hex::encode(&uncompressed[1..])
        },
        KeyType::Ed25519 => {
            // Ed25519 keys are already in their canonical form
            hex::encode(public_key.clone().try_into_ed25519()?.to_bytes())
        },
        key_type => return Err(format!("Unsupported key type: {:?}", key_type).into()),
    };

    println!("PeerID: {}", public_key.to_peer_id().to_base58());

    Ok(format!("enode://{}@{}:{}?discport=30304", uncompressed_key, ip, port))
}

/// Parse command line arguments with defaults
fn parse_args() -> (String, String, String) {
    let args: Vec<String> = std::env::args().collect();

    let peer_id_str = args.get(1)
        .cloned()
        .unwrap_or_else(|| {
            eprintln!("Usage: {} <peer_id> [ip] [port]", args[0]);
            std::process::exit(1);
        });

    let ip = args.get(2).cloned().unwrap_or_else(|| "127.0.0.1".to_string());
    let port = args.get(3).cloned().unwrap_or_else(|| "4001".to_string());

    // Validate port is numeric
    if port.parse::<u16>().is_err() {
        eprintln!("Error: Port must be a valid number between 1-65535");
        std::process::exit(1);
    }

    (peer_id_str, ip, port)
}

fn main() {
    let (peer_id_str, ip, port) = parse_args();

    // Parse and validate PeerID
    let peer_id = match peer_id_str.parse::<PeerId>() {
        Ok(id) => id,
        Err(e) => {
            eprintln!("Error: Invalid PeerId '{}': {}", peer_id_str, e);
            std::process::exit(1);
        }
    };

    let pk_hex = hex::encode(peer_id.to_bytes());
    println!("PeerID public key: {}", pk_hex);

    // Convert to enode format
    match public_key_to_enode(&pk_hex, &ip, &port) {
        Ok(enode) => println!("Enode URL: {}", enode),
        Err(e) => {
            eprintln!("Error converting public key to enode: {}", e);
            std::process::exit(1);
        }
    }
}