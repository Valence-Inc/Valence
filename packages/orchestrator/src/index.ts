// Phase 5: LangGraph orchestrator.
//
// Pipeline: CEO (intake) → COO (planning) → CFO (risk gate) → Executor → Synthesizer
//
// The orchestrator NEVER imports stellar-sdk. It depends only on the
// PaymentProvider interface from @kasuwa/payment-hub so other payment
// rails can plug in later.
//
// Uses NVIDIA NIM for LLM calls via LangGraph.

export {};
