#![cfg(test)]

use soroban_sdk::{testutils::Address as _, Address, Env, String};

use crate::our_token::{SibToken, SibTokenClient};
struct SetUpResult<'a> {
    env: Env,
    client: SibTokenClient<'a>,
    owner: Address,
    receiver: Address,
    spender: Address,
}

fn setup<'a>() -> SetUpResult<'a> {
    let env = Env::default();
    env.mock_all_auths();
    let contract_id = env.register(SibToken, ());
    let client = SibTokenClient::new(&env, &contract_id);
    let owner = Address::generate(&env);
    let receiver = Address::generate(&env);
    let spender = Address::generate(&env);

    SetUpResult {
        env,
        client,
        owner,
        receiver,
        spender,
    }
}

#[test]
fn test_name() {
    let setup_result = setup();
    let name = setup_result.client.name();
    let token_name = String::from_str(&setup_result.env, "SibToken");
    assert_eq!(name, token_name);
}

#[test]
fn test_symbol() {
    let setup_result = setup();
    let name = setup_result.client.symbol();
    let token_name = String::from_str(&setup_result.env, "SIB");
    let not_token_name = String::from_str(&setup_result.env, "Sib");
    assert_eq!(name, token_name);
    assert_ne!(name, not_token_name);
}

#[test]
fn test_decimal() {
    let setup_result = setup();
    let decimal = setup_result.client.decimals();
    let token_decimal = 18;
    assert_eq!(decimal, token_decimal);
}

#[test]
fn test_init_mints_supply() {
    let setup_result = setup();
    let owner = setup_result.owner.clone();

    setup_result.client.init(&owner, &1_000);
    assert_eq!(setup_result.client.balance(&owner), 1_000);
}

#[test]
fn test_transfer() {
    let setup_result = setup();
    let owner = setup_result.owner.clone();
    let receiver = setup_result.receiver.clone();

    setup_result.client.init(&owner, &1_000);
    setup_result.client.transfer(&owner, &receiver, &250);

    assert_eq!(setup_result.client.balance(&owner), 750);
    assert_eq!(setup_result.client.balance(&receiver), 250);
}

#[test]
fn test_transfer_from() {
    let setup_result = setup();
    let owner = setup_result.owner.clone();
    let spender = setup_result.spender.clone();
    let receiver = setup_result.receiver.clone();

    setup_result.client.init(&owner, &1_000);
    setup_result.client.approve(&owner, &spender, &400, &100);
    setup_result
        .client
        .transfer_from(&spender, &owner, &receiver, &300);

    assert_eq!(setup_result.client.balance(&owner), 700);
    assert_eq!(setup_result.client.balance(&receiver), 300);
    assert_eq!(setup_result.client.allowance(&owner, &spender), 100);
}

#[test]
fn test_burn() {
    let setup_result = setup();
    let owner = setup_result.owner.clone();

    setup_result.client.init(&owner, &1_000);
    setup_result.client.burn(&owner, &200);

    assert_eq!(setup_result.client.balance(&owner), 800);
}

#[test]
fn test_burn_from() {
    let setup_result = setup();
    let owner = setup_result.owner.clone();
    let spender = setup_result.spender.clone();

    setup_result.client.init(&owner, &1_000);
    setup_result.client.approve(&owner, &spender, &500, &100);
    setup_result.client.burn_from(&spender, &owner, &275);

    assert_eq!(setup_result.client.balance(&owner), 725);
    assert_eq!(setup_result.client.allowance(&owner, &spender), 225);
}

#[test]
fn test_mint() {
    let setup_result = setup();
    let owner = setup_result.owner.clone();
    let receiver = setup_result.receiver.clone();

    setup_result.client.init(&owner, &1_000);
    setup_result.client.mint(&receiver, &500);

    assert_eq!(setup_result.client.balance(&receiver), 500);
    assert_eq!(setup_result.client.balance(&owner), 1_000);
}
