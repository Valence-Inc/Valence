#!/usr/bin/env bash
# Deploy Soroban contracts to Stellar testnet.
#
# Prerequisites:
#   - stellar CLI installed
#   - STELLAR_SECRET_KEY set (deployer account, funded on testnet)
#   - Contracts built: cargo build --release --target wasm32v1-none
#
# Outputs:
#   - Writes ESCROW_CONTRACT_ID and REPUTATION_CONTRACT_ID to .env

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(cd "$SCRIPT_DIR/.." && pwd)"

echo "=== Oja Contract Deployment ==="
echo "Network: testnet"
echo ""
echo "TODO(Phase 2): Build and deploy contracts"
echo "  1. cargo build --release --target wasm32v1-none --manifest-path contracts/Cargo.toml"
echo "  2. stellar contract deploy --wasm target/.../oja_reputation.wasm --network testnet"
echo "  3. stellar contract deploy --wasm target/.../oja_escrow.wasm --network testnet"
echo "  4. Initialize reputation contract"
echo "  5. Initialize escrow contract with reputation contract ID"
echo "  6. Write contract IDs to $ROOT_DIR/.env"
