use crate::protocol::*;
use anyhow::Result;
use regex::Regex;
use serde_json::json;

pub fn handle(re: &Regex, params: &Params) -> Result<ResponseData> {
    let s = re.replace_all(&params.test_content, &params.replacement);

    Ok(ResponseData {
        matched: true,
        result: json!({
            "Text": s.to_string()
        }),
    })
}
