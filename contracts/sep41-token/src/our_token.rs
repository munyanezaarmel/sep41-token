use crate::{
    error::ContractError,
    events::{Approval, Burn, Mint, Transfer},
    storage::{AllowanceKey, DataKey},
    token_trait::TokenInterface,
};
use soroban_sdk::{contract, contractimpl, Address, Env, IntoVal, MuxedAddress, String};

#[contract]
pub struct SibToken;

#[contractimpl]
impl SibToken {
    pub fn init(env: Env, admin: Address, amount: i128) -> Result<(), ContractError> {
        admin.require_auth();

        if env.storage().persistent().has(&DataKey::Minter) {
            return Err(ContractError::AlreadyInitialized);
        }

        env.storage().persistent().set(&DataKey::Minter, &admin);
        Self::mint_internal(env.clone(), admin.clone(), amount)?;
        Ok(())
    }

    pub fn allowance(env: Env, from: Address, spender: Address) -> i128 {
        env.storage()
            .persistent()
            .get(&DataKey::Allowance(AllowanceKey { from, spender }))
            .unwrap_or(0)
    }

    pub fn approve(
        env: Env,
        from: Address,
        spender: Address,
        amount: i128,
        live_until_ledger: u32,
    ) -> Result<(), ContractError> {
        from.require_auth();

        if amount < 0 {
            return Err(ContractError::InvalidAmount);
        }

        let key = DataKey::Allowance(AllowanceKey { from: from.clone(), spender: spender.clone() });
        env.storage().persistent().set(&key, &amount);

        Approval {
            from,
            spender,
            amount,
            live_until_ledger: live_until_ledger.into_val(&env),
        }
        .publish(&env);

        Ok(())
    }

    pub fn balance(env: Env, id: Address) -> i128 {
        env.storage()
            .persistent()
            .get(&DataKey::Balance(id))
            .unwrap_or(0)
    }

    pub fn transfer(
        env: Env,
        from: Address,
        to: MuxedAddress,
        amount: i128,
    ) -> Result<(), ContractError> {
        from.require_auth();

        if amount < 0 {
            return Err(ContractError::InvalidAmount);
        }

        let to_address = to.address();
        let sender_balance = Self::balance(env.clone(), from.clone());
        let receiver_balance = Self::balance(env.clone(), to_address.clone());

        if sender_balance < amount {
            return Err(ContractError::InsufficientFunds);
        }

        Self::set_balance(&env, from.clone(), sender_balance - amount);
        Self::set_balance(&env, to_address.clone(), receiver_balance + amount);

        Transfer {
            from,
            to: to_address,
            amount,
        }
        .publish(&env);

        Ok(())
    }

    pub fn transfer_from(
        env: Env,
        spender: Address,
        from: Address,
        to: Address,
        amount: i128,
    ) -> Result<(), ContractError> {
        spender.require_auth();

        if amount < 0 {
            return Err(ContractError::InvalidAmount);
        }

        let allowance_amount = Self::allowance(env.clone(), from.clone(), spender.clone());
        if allowance_amount < amount {
            return Err(ContractError::AllowanceExceeded);
        }

        let from_balance = Self::balance(env.clone(), from.clone());
        if from_balance < amount {
            return Err(ContractError::InsufficientFunds);
        }

        let remaining_allowance = allowance_amount - amount;
        let allowance_key = DataKey::Allowance(AllowanceKey { from: from.clone(), spender: spender.clone() });
        env.storage().persistent().set(&allowance_key, &remaining_allowance);

        Self::transfer(env.clone(), from.clone(), to.clone().into(), amount)
    }

    pub fn burn(env: Env, from: Address, amount: i128) -> Result<(), ContractError> {
        from.require_auth();

        if amount < 0 {
            return Err(ContractError::InvalidAmount);
        }

        let from_balance = Self::balance(env.clone(), from.clone());
        if from_balance < amount {
            return Err(ContractError::InsufficientFunds);
        }

        Self::set_balance(&env, from.clone(), from_balance - amount);
        let total_supply = Self::total_supply(env.clone());
        Self::set_total_supply(&env, total_supply - amount);

        Burn { from, amount }.publish(&env);
        Ok(())
    }

    pub fn burn_from(
        env: Env,
        spender: Address,
        from: Address,
        amount: i128,
    ) -> Result<(), ContractError> {
        spender.require_auth();

        if amount < 0 {
            return Err(ContractError::InvalidAmount);
        }

        let allowance_amount = Self::allowance(env.clone(), from.clone(), spender.clone());
        if allowance_amount < amount {
            return Err(ContractError::AllowanceExceeded);
        }

        let from_balance = Self::balance(env.clone(), from.clone());
        if from_balance < amount {
            return Err(ContractError::InsufficientFunds);
        }

        let remaining_allowance = allowance_amount - amount;
        let allowance_key = DataKey::Allowance(AllowanceKey { from: from.clone(), spender: spender.clone() });
        env.storage().persistent().set(&allowance_key, &remaining_allowance);

        Self::set_balance(&env, from.clone(), from_balance - amount);
        let total_supply = Self::total_supply(env.clone());
        Self::set_total_supply(&env, total_supply - amount);

        Burn { from, amount }.publish(&env);
        Ok(())
    }

    pub fn mint(env: Env, to: Address, amount: i128) -> Result<(), ContractError> {
        let minter = Self::minter(env.clone())?;
        minter.require_auth();
        Self::mint_internal(env, to, amount)
    }

    pub fn decimals(_env: Env) -> u32 {
        18
    }

    pub fn name(env: Env) -> String {
        String::from_str(&env, "SibToken")
    }

    pub fn symbol(env: Env) -> String {
        String::from_str(&env, "SIB")
    }
}

impl SibToken {
    fn mint_internal(env: Env, to: Address, amount: i128) -> Result<(), ContractError> {
        if amount < 0 {
            return Err(ContractError::InvalidAmount);
        }

        let current_balance = Self::balance(env.clone(), to.clone());
        Self::set_balance(&env, to.clone(), current_balance + amount);

        let total_supply = Self::total_supply(env.clone());
        Self::set_total_supply(&env, total_supply + amount);

        Mint { to, amount }.publish(&env);
        Ok(())
    }

    fn set_balance(env: &Env, id: Address, amount: i128) {
        env.storage().persistent().set(&DataKey::Balance(id), &amount);
    }

    fn total_supply(env: Env) -> i128 {
        env.storage()
            .persistent()
            .get(&DataKey::TotalSupply)
            .unwrap_or(0)
    }

    fn set_total_supply(env: &Env, amount: i128) {
        env.storage().persistent().set(&DataKey::TotalSupply, &amount);
    }

    fn minter(env: Env) -> Result<Address, ContractError> {
        env.storage()
            .persistent()
            .get(&DataKey::Minter)
            .ok_or(ContractError::NotInitialized)
    }
}

impl TokenInterface for SibToken {
    fn allowance(env: Env, from: Address, spender: Address) -> i128 {
        Self::allowance(env, from, spender)
    }

    fn approve(
        env: Env,
        from: Address,
        spender: Address,
        amount: i128,
        live_until_ledger: u32,
    ) -> Result<(), ContractError> {
        Self::approve(env, from, spender, amount, live_until_ledger)
    }

    fn balance(env: Env, id: Address) -> i128 {
        Self::balance(env, id)
    }

    fn transfer(
        env: Env,
        from: Address,
        to: MuxedAddress,
        amount: i128,
    ) -> Result<(), ContractError> {
        Self::transfer(env, from, to, amount)
    }

    fn transfer_from(
        env: Env,
        spender: Address,
        from: Address,
        to: Address,
        amount: i128,
    ) -> Result<(), ContractError> {
        Self::transfer_from(env, spender, from, to, amount)
    }

    fn burn(env: Env, from: Address, amount: i128) -> Result<(), ContractError> {
        Self::burn(env, from, amount)
    }

    fn burn_from(
        env: Env,
        spender: Address,
        from: Address,
        amount: i128,
    ) -> Result<(), ContractError> {
        Self::burn_from(env, spender, from, amount)
    }

    fn mint(env: Env, to: Address, amount: i128) -> Result<(), ContractError> {
        Self::mint(env, to, amount)
    }

    fn decimals(env: Env) -> u32 {
        Self::decimals(env)
    }

    fn name(env: Env) -> String {
        Self::name(env)
    }

    fn symbol(env: Env) -> String {
        Self::symbol(env)
    }
}
