import type { TaskPlan, ContractTaskId } from "@kasuwa/shared";

// ─── Result types ──────────────────────────────────────────

export interface TxResult {
  /** Stellar transaction hash. */
  txHash: string;
  /** Ledger sequence number. */
  ledgerSeq: number;
}

export interface CreateTaskResult extends TxResult {
  contractTaskId: ContractTaskId;
}

// ─── Param types ───────────────────────────────────────────

export interface CreateTaskParams {
  plan: TaskPlan;
  /** Stellar address of the client (payer). */
  clientAddress: string;
  /** SAC token contract address. */
  tokenAddress: string;
}

export interface AssignStepParams {
  contractTaskId: ContractTaskId;
  stepIndex: number;
  workerAddress: string;
}

export interface SettleStepParams {
  contractTaskId: ContractTaskId;
  stepIndex: number;
}

export interface FailStepParams {
  contractTaskId: ContractTaskId;
  stepIndex: number;
}

export interface CancelTaskParams {
  contractTaskId: ContractTaskId;
}

// ─── Provider interface ────────────────────────────────────

/**
 * Abstraction over the payment rail. The orchestrator depends on this
 * interface, never on stellar-sdk directly.
 *
 * Phase 3 provides the Stellar/Soroban implementation.
 * Swapping to a different chain means implementing this interface.
 */
export interface PaymentProvider {
  createTask(params: CreateTaskParams): Promise<CreateTaskResult>;
  assignStep(params: AssignStepParams): Promise<TxResult>;
  settleStep(params: SettleStepParams): Promise<TxResult>;
  failStep(params: FailStepParams): Promise<TxResult>;
  cancelTask(params: CancelTaskParams): Promise<TxResult>;

  /** Read a worker's reputation score (basis points 0–10 000). */
  getWorkerScore(workerAddress: string): Promise<number>;
}
