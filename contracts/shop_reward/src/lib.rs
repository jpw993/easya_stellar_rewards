#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, symbol_short, Address, Env, Symbol};

use crate::balance::{read_balance, receive_balance};

use soroban_token_sdk::TokenUtils;

#[contracttype]
pub enum StorageKey {
    /// Owner. Value is an Address.
    Owner,
    /// Loyalty reward ratio. Value is an (amt_spent, loyalty_points) tuple.
    LoyaltyRewardRatio,
}

#[contract]
pub struct ShopReward;

#[contractimpl]
impl ShopReward {
    pub fn set_owner(env: Env, new_owner: Address) {
        if let Some(admin) = env
            .storage()
            .instance()
            .get::<_, Address>(&StorageKey::Owner)
        {
            admin.require_auth();
        };
        env.storage().instance().set(&StorageKey::Owner, &new_owner);
    }

    /// Return the owner address.
    pub fn get_owner(env: &Env) -> Address {
        env.storage()
            .instance()
            .get::<_, Address>(&StorageKey::Owner)
            .unwrap()
    }

    /// Set the loyalty reward ratio (amt_spent, loyalty_points)
    pub fn set_loyalty_reward_ratio(env: Env, amt_spent: i128, loyalty_points: i128) {
        let owner = ShopReward::get_owner(&env);
        owner.require_auth();

        check_nonnegative_amount(amt_spent);
        check_nonnegative_amount(loyalty_points);

        env.storage().instance().set(
            &StorageKey::LoyaltyRewardRatio,
            &(amt_spent, loyalty_points),
        );
    }

    /// Retrieve the current loyalty reward ratio
    pub fn get_loyalty_reward_ratio(env: &Env) -> (i128, i128) {
        env.storage()
            .instance()
            .get::<_, (i128, i128)>(&StorageKey::LoyaltyRewardRatio)
            .unwrap_or((0, 0)) // Return (0, 0) if not set
    }

    pub fn mint_loyalty_tokens(e: Env, to: Address, amount: i128) {
        check_nonnegative_amount(amount);
        let owner = ShopReward::get_owner(&e);
        owner.require_auth();

        receive_balance(&e, to.clone(), amount);
        TokenUtils::new(&e).events().mint(owner, to, amount);
    }

    pub fn get_loyalty_token_balance(e: Env, address: Address) -> i128 {
        read_balance(&e, address)
    }

    pub fn make_purchase(env: Env, sender: Address, product: Symbol, amount: i128) {
        // Ensure the amount is non-negative
        check_nonnegative_amount(amount);

        // Get the owner of the contract
        let owner = ShopReward::get_owner(&env);

        // Authenticate the invoker (sender)
        sender.require_auth();

        // Transfer the amount from the sender to the owner
        receive_balance(&env, owner.clone(), amount);

        // emit purchase Event
        let topics = (symbol_short!("purchased"), product, amount);
        env.events().publish(topics, amount);

        // Calculate loyalty tokens based on the loyalty reward ratio
        let (amt_spent, loyalty_points): (i128, i128) = env
            .storage()
            .instance()
            .get(&StorageKey::LoyaltyRewardRatio)
            .unwrap();

        let loyalty_tokens = (amount * loyalty_points) / amt_spent;

        // Mint loyalty tokens for the sender
        if loyalty_tokens > 0 {
            ShopReward::mint_loyalty_tokens(env.clone(), sender.clone(), loyalty_tokens);
        }
    }
}

fn check_nonnegative_amount(amount: i128) {
    if amount < 0 {
        panic!("negative amount is not allowed: {}", amount)
    }
}

mod balance;
mod storage_types;
mod test;
