use serde::{Deserialize, Serialize};
use web3::types::{Trace, TraceFilterBuilder};

#[derive(Serialize)]
struct JsonRpcRequest<T> {
    jsonrpc: String,
    method: String,
    id: u32,
    params: T,
}

#[derive(Deserialize)]
#[allow(dead_code)]
struct JsonRpcResponse<T> {
    jsonrpc: String,
    id: u32,
    result: T,
}

#[tokio::main]
async fn main() {
    let client = reqwest::Client::new();

    let filter = TraceFilterBuilder::default()
        .from_block(0x2930AF5.into())
        .to_block(0x2930AF5.into())
        .build();

    let req = JsonRpcRequest {
        jsonrpc: "2.0".to_string(),
        method: "trace_filter".to_string(),
        id: 1,
        params: vec![filter],
    };

    let res = client
        .post("https://rpcapi-tracing.fantom.network")
        .json(&req)
        .send()
        .await
        .unwrap();

    let traces: Vec<serde_json::Value> = res.json::<JsonRpcResponse<_>>().await.unwrap().result;

    println!("Length {:?}", traces.len());

    for trace in traces {
        let res = serde_json::from_value::<Trace>(trace.clone());
        if let Err(e) = res {
            println!("{}", serde_json::to_string_pretty(&trace).unwrap());
            println!("{:?}", e);
        }
    }
}
