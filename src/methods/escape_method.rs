use crate::protocol::*;
use anyhow::Result;
use regex::escape;
use serde_json::json;

pub fn handle(params: &Params) -> Result<ResponseData> {
    Ok(ResponseData {
        matched: true,
        result: json!({
            "Pattern": escape(&params.pattern)
        }),
    })
}
