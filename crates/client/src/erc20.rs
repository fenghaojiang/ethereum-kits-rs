use anyhow::Result;
use ethers::contract::abigen;
use ethers::middleware::SignerMiddleware;
use ethers::providers::Http;
use ethers::providers::Provider;
use ethers::types::Address;
use std::sync::Arc;

// https://eips.ethereum.org/EIPS/eip-20
abigen!(
    IERC20,
    r#"
    [
        function name() external view returns (string)
        function symbol() external view returns (string)
        function decimals() external view returns (uint8)
        function totalSupply() external view returns (uint256)
        function balanceOf(address user) external retures (uint256)
        
        function transfer(address _to, uint256 _value) external returns (bool success)
        function transferFrom(address _from, address _to, uint256 _value) external returns (bool success)
        function approve(address _spender, uint256 _value) external returns (bool success)
        function allowance(address _owner, address _spender) external view returns (uint256 remaining)
    ]
"#
);

#[tokio::test(flavor = "multi_thread", worker_threads = 1)]
async fn test_on_erc20_call() -> Result<()> {
    let provider_arc = Arc::new(Provider::<Http>::try_from("https://rpc.ankr.com/eth")?);
    let birdring_account = "0x162c6270266667ccf5a9ed752b5d6a2bdc0f90de".parse()?;
    let rss3_token: Address = "0xc98d64da73a6616c42117b582e832812e7b8d57f".parse()?;
    let usdt_token: Address = "0xdAC17F958D2ee523a2206206994597C13D831ec7".parse()?;
    let provider_usdt = provider_arc.clone();

    let erc20_contract = Arc::new(IERC20::new(rss3_token, provider_arc));
    let usdt_contract = Arc::new(IERC20::new(usdt_token, provider_usdt.clone()));

    let rss3_decimals = erc20_contract.decimals().call().await?;
    let rss3_name = erc20_contract.name().call().await?;
    let rss3_symbol = erc20_contract.symbol().call().await?;
    let rss3_total_supply = erc20_contract.total_supply().call().await?;
    let rss3_balance = erc20_contract.balance_of(birdring_account).call().await?;

    println!("rss3 decimals: {}", rss3_decimals);
    println!("rss3 name: {}", rss3_name);
    println!("rss3 symbol: {}", rss3_symbol);
    println!("rss3 total supply: {}", rss3_total_supply);

    let usdt_name = usdt_contract.name().call().await?;
    let usdt_symbol = usdt_contract.symbol().call().await?;

    println!("usdt name: {}", usdt_name);
    println!("usdt symbol: {}", usdt_symbol);

    println!("birdring rss3 balance: {}", rss3_balance);
    Ok(())
}
