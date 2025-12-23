# P2P PeerID to Enode Converter

A Rust utility that converts libp2p PeerIDs to Ethereum enode URLs. This tool is useful for bridging between libp2p and Ethereum networking protocols.

## Overview

This program takes a libp2p PeerID and converts it to an enode URL format, which is commonly used in Ethereum networking. It supports both secp256k1 and Ed25519 key types.

## Features

- Convert libp2p PeerIDs to enode URLs
- Support for secp256k1 and Ed25519 key types
- Configurable IP address and port
- Input validation and error handling
- Command-line interface

## Installation

1. Make sure you have Rust installed: https://rustup.rs/
2. Clone this repository
3. Build the project:

```bash
cargo build --release
```

## Usage

```bash
cargo run -- <peer_id> [ip] [port]
```

### Arguments

- `peer_id` (required): The libp2p PeerID to convert
- `ip` (optional): IP address for the enode URL (default: 127.0.0.1)
- `port` (optional): Port number for the enode URL (default: 4001)

### Examples

Convert a PeerID with default IP and port:
```bash
cargo run -- 16Uiu2HAmRjoQDv1JJMnryJgUisofcMyQeREc6nXN39drk65DiX1r
```

Convert a PeerID with custom IP and port:
```bash
cargo run -- 16Uiu2HAmRjoQDv1JJMnryJgUisofcMyQeREc6nXN39drk65DiX1r 192.168.1.100 8545
```

### Sample Output

```
PeerID public key: 01221220f4d2c1b370c45f2c0c5f5e8c0b8f7d6e5a4b3c2d1e0f9a8b7c6d5e4f3a2b1c0
PeerID: 16Uiu2HAmRjoQDv1JJMnryJgUisofcMyQeREc6nXN39drk65DiX1r
Enode URL: enode://f4d2c1b370c45f2c0c5f5e8c0b8f7d6e5a4b3c2d1e0f9a8b7c6d5e4f3a2b1c0@127.0.0.1:4001
```

## Supported Key Types

- **secp256k1**: Uses uncompressed format (removes 0x04 prefix)
- **Ed25519**: Uses canonical byte representation