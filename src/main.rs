
use anvil_core::eth::{
    transaction::{LegacyTransaction, TransactionKind, TypedTransaction},
};

use ethers_core::types::{Bytes, Signature};
use primitive_types::{H160, U256};
use std::str::FromStr;

fn main() {
    let _tx = TypedTransaction::Legacy(LegacyTransaction {
        nonce: U256::zero(),
        gas_price: U256::zero(),
        gas_limit: U256::zero(),
        kind: TransactionKind::Call(H160::from_str("0xe592427a0aece92de3edee1f18e0157c05861564").unwrap()),
        value: U256::zero(),
        input: Bytes::default(),
        signature: Signature {
            r: U256::zero(),
            s: U256::zero(),
            v: 0u64,
        }
    });    
}