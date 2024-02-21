use anyhow::{anyhow, ensure, Result};
use ethers::prelude::coins_bip39::English;
use ethers::prelude::k256::ecdsa::SigningKey;
use ethers::prelude::transaction::eip2718::TypedTransaction;
use ethers::prelude::Signature;
use ethers::signers::{LocalWallet, MnemonicBuilder, Signer, Wallet};
use ethers::types::Address as WalletAddress;

pub struct Account {
    pub wallet: Wallet<SigningKey>,
    pub address: WalletAddress,
}

impl Account {
    pub fn new(key: KeyOpt) -> Result<Self> {
        ensure!(!key.phrase_key.is_empty() || !key.private_key.is_empty());
        if !key.phrase_key.is_empty() && !key.private_key.is_empty() {
            return Err(anyhow!(
                "private key and phrase key are ambigous, one is enough"
            ));
        }

        if key.phrase_key.is_empty() {
            let wallet = MnemonicBuilder::<English>::default()
                .phrase(key.phrase_key.as_str())
                .index(0u32)
                .unwrap()
                .build()
                .unwrap();
            let acc = Account {
                address: format!("{:#x}", wallet.address())
                    .parse::<WalletAddress>()
                    .unwrap(),
                wallet,
            };

            return Ok(acc);
        }

        let wallet: LocalWallet = key.private_key.parse().unwrap();

        let acc = Account {
            address: format!("{:#x}", wallet.address())
                .parse::<WalletAddress>()
                .unwrap(),
            wallet,
        };

        Ok(acc)
    }

    pub fn account_address(&self) -> Result<String> {
        Ok(format!("{:#x}", &self.address))
    }

    pub async fn sign_message(self, msg: &str) -> Result<Signature> {
        let res = self.wallet.sign_message(msg).await?;
        Ok(res)
    }

    pub async fn sign_tx(self, tx: &TypedTransaction) -> Result<Signature> {
        let res = self.wallet.sign_transaction(tx).await?;
        Ok(res)
    }
}

#[derive(Debug, Default)]
pub struct KeyOpt {
    pub(crate) private_key: String,
    pub(crate) phrase_key: String,
}

impl KeyOpt {
    pub fn new_with_private_key(private_key: String) -> Self {
        Self {
            private_key,
            ..KeyOpt::default()
        }
    }

    pub fn new_with_phrase_key(phrase_key: String) -> Self {
        Self {
            phrase_key,
            ..KeyOpt::default()
        }
    }
}
