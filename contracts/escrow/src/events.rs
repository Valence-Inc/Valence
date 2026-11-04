use soroban_sdk::{contractevent, Address, BytesN, Env};

/// Contract event emitted when a new task is created and funds are escrowed.
///
/// Topics: `("task", "created")`
/// Data: `(task_id, client, token, total_budget, step_count)`
#[contractevent(topics = ["task", "created"])]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TaskCreatedEvent {
    pub task_id: BytesN<32>,
    pub client: Address,
    pub token: Address,
    pub total_budget: i128,
    pub step_count: u32,
}

pub fn emit_task_created(
    env: &Env,
    task_id: &BytesN<32>,
    client: &Address,
    token: &Address,
    total_budget: i128,
    step_count: u32,
) {
    TaskCreatedEvent {
        task_id: task_id.clone(),
        client: client.clone(),
        token: token.clone(),
        total_budget,
        step_count,
    }
    .publish(env);
}
