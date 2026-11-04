use kasuwa_common::{
    DataKey, EscrowError, StepRecord, TaskRecord, INSTANCE_BUMP_AMOUNT,
    INSTANCE_LIFETIME_THRESHOLD, PERSISTENT_BUMP_AMOUNT, PERSISTENT_LIFETIME_THRESHOLD,
};
use soroban_sdk::{Address, BytesN, Env};

/// Store contract admin address in instance storage.
pub fn set_admin(env: &Env, admin: &Address) {
    env.storage().instance().set(&DataKey::Admin, admin);
}

/// Retrieve contract admin address from instance storage.
pub fn get_admin(env: &Env) -> Result<Address, EscrowError> {
    env.storage()
        .instance()
        .get(&DataKey::Admin)
        .ok_or(EscrowError::NotInitialized)
}

/// Check if contract admin has been initialized.
pub fn has_admin(env: &Env) -> bool {
    env.storage().instance().has(&DataKey::Admin)
}

/// Check if a task exists in persistent storage.
pub fn has_task(env: &Env, task_id: &BytesN<32>) -> bool {
    let key = DataKey::Task(task_id.clone());
    env.storage().persistent().has(&key)
}

/// Retrieve a task record from persistent storage.
pub fn get_task(env: &Env, task_id: &BytesN<32>) -> Result<TaskRecord, EscrowError> {
    let key = DataKey::Task(task_id.clone());
    env.storage()
        .persistent()
        .get(&key)
        .ok_or(EscrowError::TaskNotFound)
}

/// Store or update a task record in persistent storage.
pub fn set_task(env: &Env, task_id: &BytesN<32>, record: &TaskRecord) {
    let key = DataKey::Task(task_id.clone());
    env.storage().persistent().set(&key, record);
}

/// Retrieve a step record from persistent storage.
pub fn get_step(env: &Env, task_id: &BytesN<32>, step_idx: u32) -> Result<StepRecord, EscrowError> {
    let key = DataKey::Step(task_id.clone(), step_idx);
    env.storage()
        .persistent()
        .get(&key)
        .ok_or(EscrowError::StepNotFound)
}

/// Store or update a step record in persistent storage.
pub fn set_step(env: &Env, task_id: &BytesN<32>, step_idx: u32, step: &StepRecord) {
    let key = DataKey::Step(task_id.clone(), step_idx);
    env.storage().persistent().set(&key, step);
}

/// Extend TTL for a task entry and all of its associated step entries in persistent storage.
///
/// Follows the documented TTL policy:
/// - Threshold: PERSISTENT_LIFETIME_THRESHOLD (~1 day)
/// - Bump: PERSISTENT_BUMP_AMOUNT (~30 days)
pub fn extend_ttl_task(env: &Env, task_id: &BytesN<32>, step_count: u32) {
    let task_key = DataKey::Task(task_id.clone());
    env.storage().persistent().extend_ttl(
        &task_key,
        PERSISTENT_LIFETIME_THRESHOLD,
        PERSISTENT_BUMP_AMOUNT,
    );

    for i in 0..step_count {
        let step_key = DataKey::Step(task_id.clone(), i);
        env.storage().persistent().extend_ttl(
            &step_key,
            PERSISTENT_LIFETIME_THRESHOLD,
            PERSISTENT_BUMP_AMOUNT,
        );
    }
}

/// Extend TTL for contract instance storage.
pub fn extend_ttl_instance(env: &Env) {
    env.storage()
        .instance()
        .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);
}
