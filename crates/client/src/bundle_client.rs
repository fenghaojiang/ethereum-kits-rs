use std::fmt::format;

use crate::{builders::BlockBuilderEndpoint, json_rpc};
use anyhow::{anyhow, Error};
use ethers::{
    middleware::builder,
    types::transaction::{eip2718::TypedTransaction, response},
};
use futures::{future::{join_all, try_join_all}, stream::FuturesUnordered, try_join};
use reqwest::{header, Client, Response, StatusCode};
use serde::{Deserialize, Serialize};
use serde_json::{error, Value};
use serde_with::skip_serializing_none;
use tokio::{
    io::join,
    task::{self, JoinSet},
};
use tracing::{info, warn, error};

#[derive(Default)]
pub struct BundleClient {
    client: Client,
}

#[skip_serializing_none]
#[derive(Default, Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BundleParams {
    pub txs: Vec<String>,
    pub block_number: String,

    // Option params
    pub min_timestamp: Option<u64>,
    pub max_timestamp: Option<u64>,
    pub reverting_tx_hashes: Option<Vec<String>>,
    pub replacement_uuid: Option<String>,
}

impl BundleClient {
    pub fn new() -> Self {
        let mut headers = header::HeaderMap::new();
        headers.insert(
            "Content-Type",
            header::HeaderValue::from_static("application/json"),
        );

        Self {
            client: Client::builder().default_headers(headers).build().unwrap(),
            ..Default::default()
        }
    }

    /// return bundle hash if exist
    pub async fn send_bundle(
        &self,
        raw_txns: Vec<String>,
        target_block: u64,
        builder_endpoints: Vec<BlockBuilderEndpoint>,
    ) -> anyhow::Result<()> {
        if raw_txns.is_empty() {
            error!("empty txns is not allowed");
            return Err(anyhow!("empty txns"));
        }

        let req_body = serde_json::to_string_pretty(&BundleParams {
            txs: raw_txns,
            block_number: format!("{:#x}", target_block),
            ..Default::default()
        })?;

        let mut tasks = JoinSet::new();

        for endpoints in builder_endpoints.iter() {
            for mainnet_url in endpoints.mainnet_endpoint() {
                let cli = self.client.clone();
                let bundle_req = json_rpc::to_json_rpc(req_body.clone());
                
                let sub_task = async move {
                    let bundle_req_copy = bundle_req.clone();
                    let url: String = mainnet_url.clone();
                    let resp = cli.post::<String>(mainnet_url).body(bundle_req).send().await;

                    match resp {
                        Ok(res) => {
                            if res.status() != StatusCode::OK {
                                error!("failed to send bundle to endpoint: {}, bundle request param: {}", url, bundle_req_copy);
                                return Err(anyhow!("failed to send bundle to endpoint"));
                            }  

                            let res_str = match res.text().await {
                                Ok(res_string) => {
                                    res_string
                                },
                                Err(e) => {
                                    error!("failed to convert resp text: {}", e);
                                    return Err(anyhow!("failed to convert resp text"))
                                }
                            };
                             
                            let resp_json :Value = serde_json::from_str(res_str.as_str())?;
                            if let Some(bundlehash) = resp_json["result"]["bundleHash"].as_str() {
                                info!("send bundle to endpoint: {}, bundle hash: {}", url, bundlehash);
                            } else {
                                info!("send bundle to endpoint: {}, resp: {}", url, resp_json);
                            }

                            Ok(())
                        },
                        Err(e) => {
                            error!("failed to send bundle to endpoint: {}, bundle request param: {}, error: {}", url, bundle_req_copy, e.to_string());
                            return Err(anyhow!("failed to send bundle to endpoint"));
                        }
                    }
                };

                tasks.spawn(sub_task);
            }
        }

        while let Some(_) = tasks.join_next().await {}
        Ok(())        
    }
    
    
}

#[test]
fn test_on_bundle_client() {
    let cli = BundleClient::new();
}

#[test]
fn test_on_parse_bundle_params_json() {
    let raw = r#"
    {   
        "txs":["0x2861af00746a68b408548348cb2168865dfa224ceaaa1c4e838e251bc771b965"],
        "blockNumber": "0x123456",
        "minTimestamp": 19373051,
        "maxTimestamp": 19373051,
        "revertingTxHashes": ["0x2861af00746a68b408548348cb2168865dfa224ceaaa1c4e838e251bc771b965"],
        "replacementUuid": "replacement_uuid"
    }"#;

    let params: BundleParams = serde_json::from_str(raw).unwrap();
    println!("{:?}", params);

    let json_str = serde_json::to_string_pretty(&BundleParams {
        txs: vec!["0x2861af00746a68b408548348cb2168865dfa224ceaaa1c4e838e251bc771b965".to_string()],
        block_number: "0x123455".to_string(),
        ..Default::default()
    })
    .unwrap();
    println!("{}", json_str);

    let raw_bundle_json = r#"
    {"txs":[
            "0x02f871053a85055ae82600850c1b71080082520894c101c69340feb4d0c474bf8fc34f5266f3de8a158504a817c80080c001a0243cde2c028c7401912bbc1bdcb08f14eb8078be1a198113f88cd5578f36e79ca01614dfd3e380201fa1cfc73d70476aac902c0e4ec86c570e31de7b689c9ec944",
            "0x02f871053b85055ae82600850c570bd20082520894c101c69340feb4d0c474bf8fc34f5266f3de8a158504a817c80080c001a0fa6f4586ef526907b8761a1f8a518c37fb4613e2214482e2ed4131ba60e44315a045c210e26710a520de4c62d677e11a48845d6836db723420ea2286ed60d3ae93"],
        "blockNumber":"0xa2740a"
    }
    "#.to_string();

    let request_params = json_rpc::to_json_rpc(raw_bundle_json);
    println!("{:?}", request_params);
}
