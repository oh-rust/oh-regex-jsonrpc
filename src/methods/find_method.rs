use crate::protocol::*;
use crate::utils::highlight::build;
use anyhow::Result;
use regex::Regex;
use serde_json::json;

pub fn handle(re: &Regex, params: &Params) -> Result<ResponseData> {
    if let Some(m) = re.find(&params.test_content) {
        Ok(ResponseData {
            matched: true,
            highlight: Some(build(re, &params.test_content)),
            result: json!({
                "Text": m.as_str(),
                "Start": m.start(),
                "End": m.end()
            }),
        })
    } else {
        Ok(ResponseData {
            matched: false,
            result: json!({}),
            highlight: None,
        })
    }
}
