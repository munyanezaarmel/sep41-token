use crate::error::ContractError;
use soroban_sdk::{Address, Env, MuxedAddress, String};

#[allow(dead_code)]
pub trait TokenInterface {
    /// Returns the allowance for `spender` to transfer from `from`.
    fn allowance(env: Env, from: Address, spender: Address) -> i128;

    /// Set the allowance by `amount` for `spender` to transfer/burn from `from`.
    fn approve(
        env: Env,
        from: Address,
        spender: Address,
        amount: i128,
        live_until_ledger: u32,
    ) -> Result<(), ContractError>;

    /// Returns the balance of `id`.
    fn balance(env: Env, id: Address) -> i128;

    /// Transfer `amount` from `from` to `to`.
    fn transfer(
        env: Env,
        from: Address,
        to: MuxedAddress,
        amount: i128,
    ) -> Result<(), ContractError>;

    /// Transfer `amount` from `from` to `to`, consuming the allowance of `spender`.
    fn transfer_from(
        env: Env,
        spender: Address,
        from: Address,
        to: Address,
        amount: i128,
    ) -> Result<(), ContractError>;

    /// Burn `amount` from `from`.
    fn burn(env: Env, from: Address, amount: i128) -> Result<(), ContractError>;

    /// Burn `amount` from `from`, consuming the allowance of `spender`.
    fn burn_from(
        env: Env,
        spender: Address,
        from: Address,
        amount: i128,
    ) -> Result<(), ContractError>;

    /// Mint `amount` to `to`.
    fn mint(env: Env, to: Address, amount: i128) -> Result<(), ContractError>;

    /// Returns the number of decimals used to represent amounts of this token.
    fn decimals(env: Env) -> u32;

    /// Returns the name for this token.
    fn name(env: Env) -> String;

    /// Returns the symbol for this token.
    fn symbol(env: Env) -> String;
}
