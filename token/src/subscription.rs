use soroban_sdk::{Address, Env};
use crate::storage_types::{DataKey, Subscription, SubscriptionDataKey};

pub fn read_subscription(e: &Env, from: &Address, spender: &Address) -> Subscription {
    let key = DataKey::Subscription(SubscriptionDataKey {
        spender: spender.clone(),
        from: from.clone(),
    });

    if let Some(subscription) = e.storage().get(&key) {
        subscription.unwrap()
    } else {
        Subscription {
            spender: spender.clone(),
            from: from.clone(),
            amount: 0,
            period: 0,
            last_charge: 0,
        }
    }
}

pub fn write_subscription(e: &Env, from: Address, spender: Address, new_subscription: Subscription) {
    let key = DataKey::Subscription(SubscriptionDataKey {
        spender: spender.clone(),
        from: from.clone(),
    });
    e.storage().set(&key, &new_subscription);
}

pub fn clear_subscription(e: &Env, from: Address, spender: Address) {
    let key = DataKey::Subscription(SubscriptionDataKey {
        spender: spender.clone(),
        from: from.clone(),
    });
    e.storage().remove(&key);
}

pub fn charge_subscription(e: &Env, subscription: &Subscription) {
    let mut updated_subscription = subscription.clone();
    if updated_subscription.amount == 0 {
        panic!("Subscription if off");
    }

    // TODO: like in the subscribe method, check for possible overflows if possible
    let valid_next_try = updated_subscription.last_charge + updated_subscription.period as u64;
    if e.ledger().timestamp() < valid_next_try {
        panic!("Subscription period is not completed");
    }

    updated_subscription.last_charge = e.ledger().timestamp();

    write_subscription(e, updated_subscription.from.clone(), updated_subscription.spender.clone(), updated_subscription);
}
