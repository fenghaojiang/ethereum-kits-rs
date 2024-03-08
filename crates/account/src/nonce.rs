use ethers::{core::types::U256, types::Address};
use std::sync::atomic::AtomicU64;
use std::sync::atomic::Ordering::SeqCst;
use std::{fmt::format, sync::Arc};
use tokio::task::JoinSet;

#[derive(Default, Debug)]
pub struct AccountNonce {
    pub account_address: Address,
    nonce: AtomicU64,
}

impl AccountNonce {
    pub fn new(account_address: Address) -> Self {
        Self {
            account_address: account_address,
            ..Default::default()
        }
    }

    pub fn update_nonce(&self, new_nonce: u64) {
        self.nonce.fetch_max(new_nonce, SeqCst);
    }

    pub fn get_nonce(&self) -> u64 {
        self.nonce.load(SeqCst)
    }
}

#[tokio::test(flavor = "multi_thread", worker_threads = 1)]
async fn test_on_default_account_nonce() {
    let zero_account = Arc::new(AccountNonce::new(Address::zero()));

    let mut task_set = JoinSet::new();
    for i in 1..1000 {
        let temp_account = Arc::clone(&zero_account);
        task_set.spawn(async move { temp_account.update_nonce(i) });
    }

    while let Some(_) = task_set.join_next().await {}

    assert_eq!(999, zero_account.get_nonce());
}
