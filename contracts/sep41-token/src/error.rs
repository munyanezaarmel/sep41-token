use soroban_sdk::contracterror;

#[contracterror]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ContractError {
    InsufficientFunds = 1,
    InvalidAmount = 2,
    AllowanceExceeded = 3,
    AlreadyInitialized = 4,
    NotInitialized = 5,
}
