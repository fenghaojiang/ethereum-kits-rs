use ethers::core::rand::thread_rng;
use ethers_signers::{
    coins_bip39::{English, Mnemonic},
    LocalWallet, MnemonicBuilder, Signer,
};

use crate::account::{Account, KeyOpt};
use anyhow::Result;

#[test]
fn test_new_account_from_private_key() {
    let mut rng = thread_rng();
    let wallet = LocalWallet::new(&mut rng);

    let expected_account_address = format!("{:#x}", wallet.address()).to_lowercase();

    println!("Address:     {}", expected_account_address);

    let private_key = format!("0x{}", hex::encode(wallet.signer().to_bytes()));

    println!("Private key: {}", private_key);

    let actual_account_address = Account::new(KeyOpt {
        private_key: private_key,
        ..KeyOpt::default()
    })
    .unwrap()
    .account_address()
    .unwrap()
    .to_lowercase();

    assert_eq!(expected_account_address, actual_account_address);
}

#[test]
fn test_new_account_from_phrase_key() -> Result<()> {
    let mut rng = thread_rng();
    let words: usize = 12;
    let phrase = Mnemonic::<English>::new_with_count(&mut rng, words)?.to_phrase();

    let builder = MnemonicBuilder::<English>::default().phrase(phrase.as_str());
    let derivation_path = "m/44'/60'/0'/0/0";
    let wallet = builder.derivation_path(derivation_path)?.build()?;
    let expected_account_address = format!("{:#x}", wallet.address()).to_lowercase();

    let actual_account_address = Account::new(KeyOpt {
        phrase_key: phrase.clone(),
        ..KeyOpt::default()
    })
    .unwrap()
    .account_address()
    .unwrap()
    .to_lowercase();

    println!("Address:     {}", expected_account_address);

    println!("Phrase: {phrase}");

    assert_eq!(expected_account_address, actual_account_address);

    Ok(())
}
