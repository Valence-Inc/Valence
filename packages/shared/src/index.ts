// ─── IDs ───────────────────────────────────────────────────
/** Off-chain UUID */
export type TaskId = string;
/** Off-chain UUID */
export type StepId = string;
/** Hex-encoded BytesN<32> for on-chain usage */
export type ContractTaskId = string;

// ─── Enums ─────────────────────────────────────────────────
export enum TaskStatus {
  Pending = "pending",
  Active = "active",
  Completed = "completed",
  Cancelled = "cancelled",
  Failed = "failed",
}

export enum StepStatus {
  Pending = "pending",
  Assigned = "assigned",
  Settled = "settled",
  Failed = "failed",
}

// ─── Step ──────────────────────────────────────────────────
export interface StepSpec {
  id: StepId;
  description: string;
  /** Capability tag the worker must have. */
  capability: string;
  /** Price in stroops. Always bigint, never number. */
  price: bigint;
  /** IDs of steps that must complete before this one. */
  dependencies: StepId[];
}

// ─── Task ──────────────────────────────────────────────────
export interface TaskPlan {
  taskId: TaskId;
  contractTaskId: ContractTaskId;
  steps: StepSpec[];
  /** Sum of all step prices. */
  totalBudget: bigint;
}

// ─── Worker ────────────────────────────────────────────────
export interface WorkerProfile {
  id: string;
  name: string;
  capabilities: string[];
  /** Default price per step in stroops. */
  pricePerStep: bigint;
  endpoint: string;
  /** Reputation score in basis points 0–10 000, read from chain. */
  reputationScore: number;
}

/**
 * Interface that all workers (AI agents or future human workers) must implement.
 */
export interface Worker {
  readonly profile: WorkerProfile;
  execute(input: unknown): Promise<unknown>;
}

// ─── Helpers ───────────────────────────────────────────────
/**
 * Derive a BytesN<32> contract task ID from an off-chain UUID.
 * Uses SHA-256 of the UTF-8 encoded UUID string.
 */
export async function deriveContractTaskId(uuid: string): Promise<ContractTaskId> {
  const encoder = new TextEncoder();
  const data = encoder.encode(uuid);
  const hashBuffer = await crypto.subtle.digest("SHA-256", data);
  const hashArray = new Uint8Array(hashBuffer);
  return Array.from(hashArray)
    .map((b) => b.toString(16).padStart(2, "0"))
    .join("");
}
