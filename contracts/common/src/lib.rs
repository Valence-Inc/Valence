#![no_std]

use soroban_sdk::contracttype;

/// On-chain step specification. Only the price matters on-chain;
/// the description and dependencies live off-chain.
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StepSpec {
    /// Price in token stroops (i128, never use floats).
    pub price: i128,
}

/// Tracks the state of a single step in an escrowed task.
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum StepStatus {
    /// Step created, no worker assigned yet.
    Pending = 0,
    /// Worker has been assigned by the arbiter (hub).
    Assigned = 1,
    /// Step completed successfully; worker has been paid.
    Settled = 2,
    /// Step failed; client has been refunded.
    Failed = 3,
}

/// Aggregate task state.
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TaskStatus {
    Active = 0,
    Completed = 1,
    Cancelled = 2,
}
