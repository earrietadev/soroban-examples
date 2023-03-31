use soroban_sdk::{contracttype, Address};

#[derive(Clone)]
#[contracttype]
pub struct AllowanceDataKey {
    pub from: Address,
    pub spender: Address,
}

#[derive(Clone)]
#[contracttype]
pub struct SubscriptionDataKey {
    pub from: Address,
    pub spender: Address,
}

#[derive(Clone)]
#[contracttype]
pub struct Subscription {
    pub spender: Address,
    pub from: Address,
    pub amount: i128,
    pub period: u32,
    pub last_charge: u64,
}

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    Subscription(SubscriptionDataKey),
    Allowance(AllowanceDataKey),
    Balance(Address),
    Nonce(Address),
    State(Address),
    Admin,
    Decimals,
    Name,
    Symbol,
}
