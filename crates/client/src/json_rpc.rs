use serde::{Deserialize, Serialize};
use anyhow::{Ok, Result};
use crate::bundle_client::BundleParams;

pub fn to_json_rpc(bundle_json: String) -> String {
    let request_body = format!(
        r#"{{"id":1,"jsonrpc":"2.0","method":"eth_sendBundle","params":[{}]}}"#,
        bundle_json
    );

    return request_body;
}



#[test] 
fn test_on_to_json_rpc() -> Result<()> {
    let bundle_json = serde_json::to_string_pretty(&BundleParams {
        txs: vec![
            "0x123123123".to_string(),
            "0x1827367816283768".to_string(),
        ],
        block_number: format!("{:#x}", 123),
        ..Default::default()
    })?;

    println!("{}", to_json_rpc(bundle_json));

    Ok(())
}



