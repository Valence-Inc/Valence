#!/usr/bin/env bash
# Phase 3: Generate TypeScript bindings from deployed contracts.
# Requires ESCROW_CONTRACT_ID and REPUTATION_CONTRACT_ID in .env.
#
# Usage: pnpm --filter @oja/contracts-client generate

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
OUT_DIR="$SCRIPT_DIR/../src/generated"

echo "TODO(Phase 3): Generate TS bindings with 'stellar contract bindings typescript'"
echo "  Output directory: $OUT_DIR"
echo "  Requires deployed contract IDs in .env"
