use crate::erc20;
use anyhow::Ok;
use anyhow::Result;
use ethers::contract::abigen;
use ethers::middleware::SignerMiddleware;
use ethers::providers::Http;
use ethers::providers::Provider;
use ethers::types::Address;
use std::sync::Arc;

abigen!(
    IOneInch,
    r#"
    [
        function getRate(address srcToken, address dstToken, bool useWrappers) view returns(uint256 weightedRate)
        function getRateToEth(address srcToken, bool useSrcWrappers) view returns(uint256 weightedRate)
    ]
    "#
);

#[tokio::test(flavor = "multi_thread", worker_threads = 1)]
async fn test_on_one_inch_oracle() -> Result<()> {
    let provider_arc = Arc::new(Provider::<Http>::try_from("https://rpc.ankr.com/eth")?);
    let rss3_token: Address = "0xc98d64da73a6616c42117b582e832812e7b8d57f".parse()?;
    let usdt_token: Address = "0xdAC17F958D2ee523a2206206994597C13D831ec7".parse()?;
    let one_inch_oracle_addr: Address = "0x0AdDd25a91563696D8567Df78D5A01C9a991F9B8".parse()?;

    let usdt_contract = Arc::new(erc20::IERC20::new(usdt_token, provider_arc.clone()));

    let usdt_decimals = usdt_contract.decimals().call().await?;

    let one_inch = Arc::new(IOneInch::new(one_inch_oracle_addr, provider_arc));
    let price = one_inch
        .get_rate(rss3_token, usdt_token, true)
        .call()
        .await?;

    println!("usdt decimals: {}", usdt_decimals.clone());

    println!(
        "rss3 token price: {}",
        price.as_u64() as f64 / 10_i32.pow(6) as f64
    );

    Ok(())
}
