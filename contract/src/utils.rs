use crate::*;

pub(crate) fn unordered_map_key_pagination<K, V>(
    map: &UnorderedMap<K, V>,
    from_index: Option<U128>,
    limit: Option<u64>,
) -> Vec<K> where K: BorshSerialize + BorshDeserialize, V: BorshSerialize + BorshDeserialize {
	let len = map.keys_as_vector().len();
	if len == 0 {
		return vec![];
	}
	let limit = limit.map(|v| v as usize).unwrap_or(usize::MAX);
	assert_ne!(limit, 0, "limit 0");
	let start_index: u128 = from_index.map(From::from).unwrap_or_default();
	assert!(
		len as u128 > start_index,
		"start_index gt len"
	);
	map
		.keys()
		.skip(start_index as usize)
		.take(limit)
		.map(|k: K| k)
		.collect()
}

pub(crate) fn unordered_set_pagination<V>(
    set: &UnorderedSet<V>,
    from_index: Option<U128>,
    limit: Option<u64>,
) -> Vec<V> where V: BorshSerialize + BorshDeserialize {
	let len = set.len();
	if len == 0 {
		return vec![];
	}
	let limit = limit.map(|v| v as usize).unwrap_or(usize::MAX);
	assert_ne!(limit, 0, "limit 0");
	let start_index: u128 = from_index.map(From::from).unwrap_or_default();
	assert!(
		len as u128 > start_index,
		"start_index gt len"
	);
	set
		.iter()
		.skip(start_index as usize)
		.take(limit)
		.map(|v| v)
		.collect()
}

/// from https://github.com/near-examples/pow-faucet/blob/94428a6afa83d8c0b8e3e7bac7f147865705d814/contract-rs/src/lib.rs

pub(crate) fn num_leading_zeros(v: &[u8]) -> u32 {
    let mut res = 0;
    for z in v.iter().map(|b| b.leading_zeros()) {
        res += z;
        if z < 8 {
            break;
        }
    }
    res
}
	
/// from https://github.com/near/near-sdk-rs/blob/e4abb739ff953b06d718037aa1b8ab768db17348/near-contract-standards/src/non_fungible_token/utils.rs#L29
/// modified with keep_amount to retain funds in contract

pub(crate) fn refund_deposit(storage_used: u64, keep_amount: Option<Balance>) {
    let required_cost = env::storage_byte_cost() * Balance::from(storage_used);
    let mut attached_deposit = env::attached_deposit();

	if let Some(keep_amount) = keep_amount {
		attached_deposit = attached_deposit.checked_sub(keep_amount).unwrap_or_else(|| env::panic_str("keep amount too large"));
	}

    assert!(
        required_cost <= attached_deposit,
        "Must attach {} yoctoNEAR to cover storage",
        required_cost,
    );

    let refund = attached_deposit - required_cost;
	// log!("refund_deposit amount {}", refund);
    if refund > 1 {
        Promise::new(env::predecessor_account_id()).transfer(refund);
    }
}

pub(crate) fn is_promise_success() -> bool {
    assert_eq!(
        env::promise_results_count(),
        1,
        "Contract expected a result on the callback"
    );
    match env::promise_result(0) {
        PromiseResult::Successful(_) => true,
        _ => false,
    }
}