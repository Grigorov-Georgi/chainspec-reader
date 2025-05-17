# Chainspec Key Reader

A lightweight CLI tool for inspecting keys in a given Polkadot chainspec file.

## Features

- Parses and displays key entries from a Polkadot chainspec.
- Supports multiple chainspec formats:
    - Standard: `genesis.raw.top`
    - Fruzhin (Java implementation): `genesis.top`
- Merges keys with human-readable names from a local `key_names.json` file.
- Skips overly large keys (e.g. `:code`) for better readability.

## Usage

```bash
cargo run -- --chainspec path/to/chainspec.json --raw