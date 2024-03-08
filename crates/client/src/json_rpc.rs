use serde::{Deserialize, Serialize};

pub fn to_json_rpc(bundle_json: String) -> String {
    let request_body = format!(
        r#"{{"id":1,"jsonrpc":"2.0","method":"eth_sendBundle","params":[{}]}}"#,
        bundle_json
    );

    return request_body;
}
