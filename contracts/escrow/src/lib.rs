#![no_std]

use oja_common::StepSpec;
use soroban_sdk::{contract, contractimpl, Address, BytesN, Env, Vec};

#[contract]
pub struct EscrowContract;

#[contractimpl]
impl EscrowContract {
    /// Lock funds for a new task. `steps` defines the price per step.
    /// The total budget (sum of step prices) is transferred from `client`
    /// to this contract. Emits event `("task", "created")`.
    pub fn create_task(
        _env: Env,
        _client: Address,
        _task_id: BytesN<32>,
        _token: Address,
        _steps: Vec<StepSpec>,
    ) {
        // Phase 1: full implementation with token transfer + storage
        todo!("Phase 1: implement create_task")
    }

    /// Assign a worker to a step. Only the arbiter (hub) may call this.
    /// Emits event `("step", "assigned")`.
    pub fn assign_step(_env: Env, _task_id: BytesN<32>, _step: u32, _worker: Address) {
        // Phase 1
        todo!("Phase 1: implement assign_step")
    }

    /// Settle a step: pay the assigned worker and call `reputation.record(true)`.
    /// Emits event `("step", "settled")`.
    pub fn settle_step(_env: Env, _task_id: BytesN<32>, _step: u32) {
        // Phase 1
        todo!("Phase 1: implement settle_step")
    }

    /// Fail a step: refund the client and call `reputation.record(false)`.
    /// Emits event `("step", "failed")`.
    pub fn fail_step(_env: Env, _task_id: BytesN<32>, _step: u32) {
        // Phase 1
        todo!("Phase 1: implement fail_step")
    }

    /// Cancel the entire task. Refund the client for all unsettled steps.
    /// Emits event `("task", "cancelled")`.
    pub fn cancel_task(_env: Env, _task_id: BytesN<32>) {
        // Phase 1
        todo!("Phase 1: implement cancel_task")
    }
}

#[cfg(test)]
mod test {
    // Phase 1: comprehensive tests
}
