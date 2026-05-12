use crate::protocol::*;
use anyhow::Result;
use regex::Regex;
use serde_json::json;

pub fn handle(re: &Regex, params: &Params) -> Result<ResponseData> {
    Ok(ResponseData {
        matched: re.is_match(&params.test_content),
        result: json!({}),
    })
}
