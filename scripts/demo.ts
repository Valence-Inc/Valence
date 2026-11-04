/**
 * End-to-end demo script.
 * Runs a complete task lifecycle: submit → plan → escrow → execute → settle.
 *
 * Usage: pnpm demo
 *
 * Prerequisites:
 *   - Postgres running (docker compose up -d)
 *   - Contracts deployed (pnpm deploy:contracts)
 *   - API server running (pnpm dev --filter @kasuwa/api)
 */

console.log("=== Kasuwa End-to-End Demo ===");
console.log("");
console.log("TODO(Phase 7): Full e2e demo");
console.log("  1. Create a testnet keypair");
console.log("  2. Fund via friendbot");
console.log("  3. POST /tasks with a sample prompt and budget");
console.log("  4. Poll GET /tasks/:id until completed");
console.log("  5. Verify ledger entries via GET /ledger");
console.log("  6. Check worker reputation scores");
