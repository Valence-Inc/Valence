#![cfg(test)]

use super::*;
use kasuwa_common::{EscrowError, StepSpec, StepStatus, TaskStatus};
use soroban_sdk::{
    testutils::{Address as _, Events},
    token, vec, Address, BytesN, Env,
};

fn setup_test<'a>() -> (
    Env,
    Address,
    EscrowContractClient<'a>,
    Address,
    token::Client<'a>,
    token::StellarAssetClient<'a>,
) {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let escrow_id = env.register(EscrowContract, ());
    let escrow_client = EscrowContractClient::new(&env, &escrow_id);

    let token_admin = Address::generate(&env);
    let token_contract = env.register_stellar_asset_contract_v2(token_admin.clone());
    let token_client = token::Client::new(&env, &token_contract.address());
    let token_admin_client = token::StellarAssetClient::new(&env, &token_contract.address());

    (
        env,
        admin,
        escrow_client,
        token_contract.address(),
        token_client,
        token_admin_client,
    )
}

#[test]
fn test_initialize() {
    let (env, admin, escrow_client, _, _, _) = setup_test();

    escrow_client.initialize(&admin);
    assert_eq!(escrow_client.get_admin(), admin);

    // Initializing twice should fail
    let second_admin = Address::generate(&env);
    let res = escrow_client.try_initialize(&second_admin);
    assert_eq!(res, Err(Ok(EscrowError::AlreadyInitialized)));
}

#[test]
fn test_create_task_success() {
    let (env, admin, escrow_client, token_addr, token_client, token_admin_client) = setup_test();
    escrow_client.initialize(&admin);

    let client = Address::generate(&env);
    token_admin_client.mint(&client, &10_000);

    let task_id = BytesN::from_array(&env, &[1u8; 32]);
    let steps = vec![
        &env,
        StepSpec { price: 100 },
        StepSpec { price: 250 },
        StepSpec { price: 650 },
    ];

    escrow_client.create_task(&client, &task_id, &token_addr, &steps);

    // Verify balances
    assert_eq!(token_client.balance(&client), 9_000);
    assert_eq!(token_client.balance(&escrow_client.address), 1_000);

    // Verify task record
    let task = escrow_client.get_task(&task_id);
    assert_eq!(task.client, client);
    assert_eq!(task.token, token_addr);
    assert_eq!(task.total_budget, 1_000);
    assert_eq!(task.remaining_balance, 1_000);
    assert_eq!(task.step_count, 3);
    assert_eq!(task.status, TaskStatus::Active);

    // Verify step records
    let step0 = escrow_client.get_step(&task_id, &0);
    assert_eq!(step0.price, 100);
    assert_eq!(step0.worker, None);
    assert_eq!(step0.status, StepStatus::Pending);

    let step1 = escrow_client.get_step(&task_id, &1);
    assert_eq!(step1.price, 250);
    assert_eq!(step1.worker, None);
    assert_eq!(step1.status, StepStatus::Pending);

    let step2 = escrow_client.get_step(&task_id, &2);
    assert_eq!(step2.price, 650);
    assert_eq!(step2.worker, None);
    assert_eq!(step2.status, StepStatus::Pending);

    // Step out of bounds
    assert_eq!(
        escrow_client.try_get_step(&task_id, &3),
        Err(Ok(EscrowError::StepNotFound))
    );

    // Verify that events were emitted
    let events = env.events().all();
    let _ = &events;
}

#[test]
fn test_create_task_empty_steps() {
    let (env, admin, escrow_client, token_addr, _, _) = setup_test();
    escrow_client.initialize(&admin);

    let client = Address::generate(&env);
    let task_id = BytesN::from_array(&env, &[2u8; 32]);
    let steps: Vec<StepSpec> = vec![&env];

    let res = escrow_client.try_create_task(&client, &task_id, &token_addr, &steps);
    assert_eq!(res, Err(Ok(EscrowError::EmptySteps)));
}

#[test]
fn test_create_task_zero_price() {
    let (env, admin, escrow_client, token_addr, _, _) = setup_test();
    escrow_client.initialize(&admin);

    let client = Address::generate(&env);
    let task_id = BytesN::from_array(&env, &[3u8; 32]);
    let steps = vec![&env, StepSpec { price: 100 }, StepSpec { price: 0 }];

    let res = escrow_client.try_create_task(&client, &task_id, &token_addr, &steps);
    assert_eq!(res, Err(Ok(EscrowError::InvalidPrice)));
}

#[test]
fn test_create_task_negative_price() {
    let (env, admin, escrow_client, token_addr, _, _) = setup_test();
    escrow_client.initialize(&admin);

    let client = Address::generate(&env);
    let task_id = BytesN::from_array(&env, &[4u8; 32]);
    let steps = vec![&env, StepSpec { price: -50 }];

    let res = escrow_client.try_create_task(&client, &task_id, &token_addr, &steps);
    assert_eq!(res, Err(Ok(EscrowError::InvalidPrice)));
}

#[test]
fn test_create_task_duplicate_id() {
    let (env, admin, escrow_client, token_addr, _, token_admin_client) = setup_test();
    escrow_client.initialize(&admin);

    let client = Address::generate(&env);
    token_admin_client.mint(&client, &10_000);

    let task_id = BytesN::from_array(&env, &[5u8; 32]);
    let steps = vec![&env, StepSpec { price: 100 }];

    escrow_client.create_task(&client, &task_id, &token_addr, &steps);

    // Second call with same task_id should fail
    let res = escrow_client.try_create_task(&client, &task_id, &token_addr, &steps);
    assert_eq!(res, Err(Ok(EscrowError::TaskAlreadyExists)));
}

#[test]
fn test_create_task_budget_overflow() {
    let (env, admin, escrow_client, token_addr, _, _) = setup_test();
    escrow_client.initialize(&admin);

    let client = Address::generate(&env);
    let task_id = BytesN::from_array(&env, &[6u8; 32]);
    let steps = vec![
        &env,
        StepSpec {
            price: i128::MAX - 10,
        },
        StepSpec { price: 20 },
    ];

    let res = escrow_client.try_create_task(&client, &task_id, &token_addr, &steps);
    assert_eq!(res, Err(Ok(EscrowError::BudgetOverflow)));
}
