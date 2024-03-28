use anyhow::{Ok, Result};
use ethers::contract::abigen;
use ethers::providers::Http;
use ethers::providers::Provider;
use ethers::types::Address;
use std::sync::Arc;

// https://eips.ethereum.org/EIPS/eip-721
abigen!(
    IERC721,
    r#"
    [
        function name() external view returns (string _name)
        function symbol() external view returns (string _symbol)
        function tokenURI(uint256 _tokenId) external view returns (string)
        function balanceOf(address _owner) external view returns (uint256)
        function ownerOf(uint256 _tokenId) external view returns (address)

        function safeTransferFrom(address _from, address _to, uint256 _tokenId, bytes data) external payable
        function safeTransferFrom(address _from, address _to, uint256 _tokenId) external payable
        function transferFrom(address _from, address _to, uint256 _tokenId) external payable
        function approve(address _approved, uint256 _tokenId) external payable
        function setApprovalForAll(address _operator, bool _approved) external
        function getApproved(uint256 _tokenId) external view returns (address)
        function isApprovedForAll(address _owner, address _operator) external view returns (bool)
    ]
"#
);

#[tokio::test(flavor = "multi_thread", worker_threads = 1)]
async fn test_on_erc721_call() -> Result<()> {
    let provider_arc = Arc::new(Provider::<Http>::try_from("https://rpc.ankr.com/eth")?);
    let birdring_account = "0x162c6270266667ccf5a9ed752b5d6a2bdc0f90de".parse()?;
    let rss3_genesis_nft: Address = "0x5452c7fb99d99fab3cc1875e9da9829cb50f7a13".parse()?;

    let rss3_nft_contract = Arc::new(IERC721::new(rss3_genesis_nft, provider_arc));

    let balance = rss3_nft_contract
        .balance_of(birdring_account)
        .call()
        .await?;
    let name = rss3_nft_contract.name().call().await?;
    let symbol = rss3_nft_contract.symbol().call().await?;

    println!("rss3 nft balance: {}", balance);
    println!("rss3 nft name: {}", name);
    println!("rss3 nft symbol: {}", symbol);

    Ok(())
}
