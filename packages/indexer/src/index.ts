// Phase 4: Soroban RPC event indexer.
//
// Polls Soroban RPC for contract events, writes rows to the Postgres
// ledger table keyed by (ledger_seq, tx_hash, event_index).
// Re-indexing is idempotent (upsert). Maintains a cursor table.
// Includes a backfill command for historical events.
//
// The indexer is the ONLY writer to the ledger table.

export {};
