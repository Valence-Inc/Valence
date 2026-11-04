#![no_std]

mod events;
mod storage;

#[cfg(test)]
mod test;

use kasuwa_common::{EscrowError, StepRecord, StepSpec, StepStatus, TaskRecord, TaskStatus};
use soroban_sdk::{contract, contractimpl, token, Address, BytesN, Env, Vec};

#[contract]
pub struct EscrowContract;

#[contractimpl]
impl EscrowContract {
    /// Initialize the escrow contract with the admin / arbiter address.
    pub fn initialize(env: Env, admin: Address) -> Result<(), EscrowError> {
        if storage::has_admin(&env) {
            return Err(EscrowError::AlreadyInitialized);
        }
        admin.require_auth();
        storage::set_admin(&env, &admin);
        storage::extend_ttl_instance(&env);
        Ok(())
    }

    /// Retrieve the admin / arbiter address.
    pub fn get_admin(env: Env) -> Result<Address, EscrowError> {
        storage::get_admin(&env)
    }

    /// Lock funds for a new task.
    ///
    /// Validations:
    /// - `client.require_auth()`
    /// - `task_id` must not already exist
    /// - `steps` must not be empty
    /// - Each step price must be strictly positive (`> 0`)
    /// - Total budget must not overflow `i128`
    ///
    /// Effects:
    /// - Transfers `total_budget` of `token` from `client` to this contract
    /// - Writes `TaskRecord` and individual `StepRecord`s to persistent storage
    /// - Extends TTL on persistent storage according to documented policy
    /// - Emits `("task", "created")` event
    pub fn create_task(
        env: Env,
        client: Address,
        task_id: BytesN<32>,
        token: Address,
        steps: Vec<StepSpec>,
    ) -> Result<(), EscrowError> {
        client.require_auth();
        storage::extend_ttl_instance(&env);

        if storage::has_task(&env, &task_id) {
            return Err(EscrowError::TaskAlreadyExists);
        }

        let step_count = steps.len();
        if step_count == 0 {
            return Err(EscrowError::EmptySteps);
        }

        let mut total_budget: i128 = 0;
        for i in 0..step_count {
            let step = steps.get(i).unwrap();
            if step.price <= 0 {
                return Err(EscrowError::InvalidPrice);
            }
            total_budget = total_budget
                .checked_add(step.price)
                .ok_or(EscrowError::BudgetOverflow)?;

            let step_record = StepRecord {
                price: step.price,
                worker: None,
                status: StepStatus::Pending,
            };
            storage::set_step(&env, &task_id, i, &step_record);
        }

        let task_record = TaskRecord {
            client: client.clone(),
            token: token.clone(),
            total_budget,
            remaining_balance: total_budget,
            step_count,
            status: TaskStatus::Active,
        };
        storage::set_task(&env, &task_id, &task_record);

        // Lock funds from client into escrow contract
        let contract_address = env.current_contract_address();
        let token_client = token::Client::new(&env, &token);
        token_client.transfer(&client, &contract_address, &total_budget);

        // Extend TTL for task and all step persistent entries
        storage::extend_ttl_task(&env, &task_id, step_count);

        // Emit task created event
        events::emit_task_created(&env, &task_id, &client, &token, total_budget, step_count);

        Ok(())
    }

    /// Read a task record from persistent storage.
    pub fn get_task(env: Env, task_id: BytesN<32>) -> Result<TaskRecord, EscrowError> {
        storage::get_task(&env, &task_id)
    }

    /// Read a step record from persistent storage.
    pub fn get_step(env: Env, task_id: BytesN<32>, step: u32) -> Result<StepRecord, EscrowError> {
        storage::get_step(&env, &task_id, step)
    }

    /// Assign a worker to a step.
    ///
    /// // TODO(contrib): Implemented in PR 3 (feat(escrow): assign, settle, fail, cancel)
    pub fn assign_step(
        _env: Env,
        _task_id: BytesN<32>,
        _step: u32,
        _worker: Address,
    ) -> Result<(), EscrowError> {
        todo!("TODO(contrib): Implemented in PR 3 (lifecycle)");
    }

    /// Settle a step: pay worker and record reputation success.
    ///
    /// // TODO(contrib): Implemented in PR 3 (feat(escrow): assign, settle, fail, cancel)
    pub fn settle_step(_env: Env, _task_id: BytesN<32>, _step: u32) -> Result<(), EscrowError> {
        todo!("TODO(contrib): Implemented in PR 3 (lifecycle)");
    }

    /// Fail a step: refund client and record reputation failure.
    ///
    /// // TODO(contrib): Implemented in PR 3 (feat(escrow): assign, settle, fail, cancel)
    pub fn fail_step(_env: Env, _task_id: BytesN<32>, _step: u32) -> Result<(), EscrowError> {
        todo!("TODO(contrib): Implemented in PR 3 (lifecycle)");
    }

    /// Cancel a task: refund all unsettled steps to client.
    ///
    /// // TODO(contrib): Implemented in PR 3 (feat(escrow): assign, settle, fail, cancel)
    pub fn cancel_task(_env: Env, _task_id: BytesN<32>) -> Result<(), EscrowError> {
        todo!("TODO(contrib): Implemented in PR 3 (lifecycle)");
    }
}
