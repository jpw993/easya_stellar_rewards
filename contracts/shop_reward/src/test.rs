#![cfg(test)]

extern crate std;

use super::*;

use soroban_sdk::testutils::Address as TestAddress;
use soroban_sdk::{symbol_short, Env};

#[test]
fn test_set_owner() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register_contract(None, ShopReward);
    let client = ShopRewardClient::new(&env, &contract_id);

    let initial_owner = Address::generate(&env);
    let new_owner = Address::generate(&env);

    client.set_owner(&initial_owner);
    assert_eq!(client.get_owner(), initial_owner);

    // Set new owner
    client.set_owner(&new_owner);
    assert_eq!(client.get_owner(), new_owner);
}

#[test]
fn test_loyalty_reward_ratio() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register_contract(None, ShopReward);
    let client = ShopRewardClient::new(&env, &contract_id);

    let owner = Address::generate(&env);
    client.set_owner(&owner);

    // Set loyalty reward ratio
    client.set_loyalty_reward_ratio(&100, &5);
    assert_eq!(client.get_loyalty_reward_ratio(), (100, 5));

    // update again
    client.set_loyalty_reward_ratio(&200, &10);
    assert_eq!(client.get_loyalty_reward_ratio(), (200, 10));
}

#[test]
fn test_make_purchase() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register_contract(None, ShopReward);
    let client = ShopRewardClient::new(&env, &contract_id);

    let owner = Address::generate(&env);
    client.set_owner(&owner);

    let customer = Address::generate(&env);
    client.set_loyalty_reward_ratio(&100, &5);
    client.make_purchase(&customer, &symbol_short!("coffee"), &200);

    // Check if loyalty tokens were minted
    let loyalty_token_bal = client.get_loyalty_token_balance(&customer);
    assert_eq!(loyalty_token_bal, 10);
}

#[test]
fn test_nonnegative_amount_check() {
    let result = std::panic::catch_unwind(|| {
        check_nonnegative_amount(-1);
    });
    assert!(result.is_err());
}
