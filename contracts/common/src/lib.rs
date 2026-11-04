#![no_std]

use soroban_sdk::{contracterror, contracttype, Address, BytesN};

/// TTL policy constants for state archiving on Stellar.
///
/// Stellar ledgers close roughly every 5 seconds:
/// - 1 day   ~= 17,280 ledgers
/// - 30 days ~= 518,400 ledgers
///
/// Persistent entries (tasks, steps) and instance entries must maintain a TTL
/// above the threshold; when read or modified, their TTL is extended by the bump amount.
pub const DAY_IN_LEDGERS: u32 = 17_280;
pub const MONTH_IN_LEDGERS: u32 = 518_400;

pub const PERSISTENT_LIFETIME_THRESHOLD: u32 = DAY_IN_LEDGERS;
pub const PERSISTENT_BUMP_AMOUNT: u32 = MONTH_IN_LEDGERS;

pub const INSTANCE_LIFETIME_THRESHOLD: u32 = DAY_IN_LEDGERS;
pub const INSTANCE_BUMP_AMOUNT: u32 = MONTH_IN_LEDGERS;

/// Storage keys for the Escrow contract.
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DataKey {
    /// Contract administrator / arbiter address (instance storage).
    Admin,
    /// Associated reputation contract address (instance storage).
    ReputationContract,
    /// Task record keyed by 32-byte task ID (persistent storage).
    Task(BytesN<32>),
    /// Step record keyed by (task_id, step_index) (persistent storage).
    Step(BytesN<32>, u32),
}

/// Errors returned by the Escrow contract.
#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum EscrowError {
    NotInitialized = 1,
    AlreadyInitialized = 2,
    TaskAlreadyExists = 3,
    TaskNotFound = 4,
    EmptySteps = 5,
    InvalidPrice = 6,
    BudgetOverflow = 7,
    StepNotFound = 8,
    StepNotPending = 9,
    StepNotAssigned = 10,
    TaskNotActive = 11,
    Unauthorized = 12,
    InsufficientBalance = 13,
}

/// On-chain step specification input for task creation.
/// Only the price matters on-chain; descriptions and DAG dependencies live off-chain.
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StepSpec {
    /// Price in token stroops (i128, strictly no floats).
    pub price: i128,
}

/// Lifecycle status for an individual step.
#[contracttype]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u32)]
pub enum StepStatus {
    /// Step created, awaiting worker assignment.
    Pending = 0,
    /// Arbiter assigned a worker to execute the step.
    Assigned = 1,
    /// Step succeeded; worker paid, reputation recorded.
    Settled = 2,
    /// Step failed; client refunded, reputation recorded.
    Failed = 3,
}

/// Aggregate lifecycle status for a task.
#[contracttype]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u32)]
pub enum TaskStatus {
    /// Task is active and accepting step assignments or settlements.
    Active = 0,
    /// All steps have reached terminal status (Settled or Failed).
    Completed = 1,
    /// Task was cancelled by client/arbiter; unsettled steps refunded.
    Cancelled = 2,
}

/// On-chain task record stored in persistent storage.
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TaskRecord {
    pub client: Address,
    pub token: Address,
    pub total_budget: i128,
    pub remaining_balance: i128,
    pub step_count: u32,
    pub status: TaskStatus,
}

/// On-chain step record stored in persistent storage.
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StepRecord {
    pub price: i128,
    pub worker: Option<Address>,
    pub status: StepStatus,
}
