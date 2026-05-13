use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Deserialize)]
pub struct RpcRequest {
    pub jsonrpc: String,
    pub id: Value,
    pub method: String,
    pub params: Params,
}

#[derive(Deserialize)]
pub struct Params {
    pub pattern: String,

    #[serde(default)]
    pub test_content: String,

    #[serde(default)]
    pub replacement: String,
}

#[derive(Serialize)]
pub struct RpcResponse {
    #[warn(unused)]
    pub jsonrpc: String,
    pub id: Value,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<ResponseData>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<RpcError>,
}

#[derive(Serialize)]
pub struct ResponseData {
    #[serde(default)]
    pub matched: bool,

    pub result: Value,

    #[serde(default)]
    pub highlight: Option<String>, // 高亮文本
}

#[derive(Serialize)]
pub struct RpcError {
    pub code: i32,
    pub message: String,
}
