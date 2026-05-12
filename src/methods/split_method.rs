use crate::protocol::*;
use anyhow::Result;
use regex::Regex;
use serde_json::json;

pub fn handle(re: &Regex, params: &Params) -> Result<ResponseData> {
    let arr: Vec<&str> = re.split(&params.test_content).collect();

    Ok(ResponseData {
        matched: true,
        result: json!(arr),
    })
}
