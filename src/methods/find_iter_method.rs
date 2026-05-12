use crate::protocol::*;
use anyhow::Result;
use regex::Regex;
use serde_json::{Value, json};

pub fn handle(re: &Regex, params: &Params) -> Result<ResponseData> {
    let arr: Vec<Value> = re
        .find_iter(&params.test_content)
        .map(|m| {
            json!({
                "Text": m.as_str(),
                "Start": m.start(),
                "End": m.end()
            })
        })
        .collect();

    Ok(ResponseData {
        matched: !arr.is_empty(),
        result: json!(arr),
    })
}
