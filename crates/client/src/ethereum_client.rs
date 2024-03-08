use std::{fmt::format, future::pending, sync::Arc, thread};

use tokio::sync::mpsc;
use tokio::task::{JoinHandle, JoinSet};

use anyhow::{anyhow, Ok, Result};
use ethers::{
    providers::{Http, JsonRpcClient, Middleware, Provider, Ws},
    types::{transaction::eip2718::TypedTransaction, Bytes, NameOrAddress, Res, H160, U256},
};

use account::{Account, AccountNonce};
use ethers::core::types::Address;
use tracing::info;

pub struct EthereumClient<M>
where
    M: JsonRpcClient,
{
    provider: Arc<Provider<M>>,
    endpoint: String,
}

pub trait EthereumClientTrait {
    async fn send_tx(&self, tx: TypedTransaction) -> Result<String>;
    async fn send_raw_tx(&self, tx_bytes: Bytes) -> Result<String>;
    async fn get_nonce(&self, addr: Address) -> Result<U256>;
}

impl EthereumClient<Http> {
    pub fn new(endpoint: String) -> Result<Self> {
        let provider = Provider::<Http>::try_from(endpoint.clone())?;
        Ok(Self {
            provider: Arc::new(provider),
            endpoint,
        })
    }

    pub fn endpoint_info(&self) -> String {
        format!("endpoint: {}", self.endpoint)
    }
}

impl EthereumClientTrait for EthereumClient<Http> {
    async fn send_tx(&self, tx: TypedTransaction) -> Result<String> {
        let pending_tx = self.provider.send_transaction(tx, None).await?;
        Ok(format!("{:#x}", pending_tx.tx_hash()))
    }

    async fn send_raw_tx(&self, tx_bytes: Bytes) -> Result<String> {
        let pending_tx = self.provider.send_raw_transaction(tx_bytes.clone()).await?;
        Ok(format!("{:#x}", pending_tx.tx_hash()))
    }

    async fn get_nonce(&self, addr: Address) -> Result<U256> {
        let nonce = self.provider.get_transaction_count(addr, None).await?;
        Ok(nonce)
    }
}

pub struct EthereumClients {
    http_clients: Vec<EthereumClient<Http>>,
}

impl EthereumClients {
    pub async fn new(rpc_endpoints: Vec<String>) -> Result<Self> {
        let mut http_clients = vec![];

        for endpoint in rpc_endpoints {
            if endpoint.starts_with("http") {
                let client = EthereumClient::new(endpoint)?;
                http_clients.push(client);
            }
        }

        if http_clients.is_empty() {
            return Err(anyhow!("no endpoints available"));
        }

        Ok(Self { http_clients })
    }

    pub async fn sign_and_send_tx(self, tx: &TypedTransaction, account: Account) -> Result<String> {
        let tx_bytes = account.sign_tx(tx).await?;
        let mut task_set = JoinSet::new();

        for client in self.http_clients {
            let temp_bytes = tx_bytes.clone();
            task_set.spawn(async move { client.send_raw_tx(temp_bytes).await });
        }

        while let Some(_) = task_set.join_next().await {}

        Ok("".to_string())
    }

    pub async fn send_tx(self, tx: &TypedTransaction) -> Result<String> {
        let mut task_set = JoinSet::new();

        for client in self.http_clients {
            let temp_tx = tx.clone();
            task_set.spawn(async move { client.send_tx(temp_tx).await });
        }

        while let Some(_) = task_set.join_next().await {}

        Ok("".to_string())
    }
}
