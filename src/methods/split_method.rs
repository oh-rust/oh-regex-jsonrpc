use crate::protocol::*;
use crate::utils::highlight::build;
use anyhow::Result;
use regex::Regex;
use serde_json::json;

pub fn handle(re: &Regex, params: &Params) -> Result<ResponseData> {
    let arr: Vec<&str> = re.split(&params.test_content).collect();

    Ok(ResponseData {
        matched: re.is_match(&params.test_content),
        highlight: Some(build(re, &params.test_content)),
        result: json!(arr),
    })
}
