use crate::protocol::*;
use anyhow::Result;
use regex::Regex;
use serde_json::json;

pub fn handle(re: &Regex, params: &Params) -> Result<ResponseData> {
    if let Some(caps) = re.captures(&params.test_content) {
        let mut groups = vec![];

        for (i, name) in re.capture_names().enumerate() {
            if let Some(m) = caps.get(i) {
                groups.push(json!({
                    "Index": i,
                    "Name": name,
                    "Text": m.as_str(),
                    "Start": m.start(),
                    "End": m.end(),
                    "Len": m.len(),
                }));
            } else {
                groups.push(json!({
                    "Index": i,
                    "Name": name,
                    "Text": "",
                    "Start": null,
                    "End": null,
                }));
            }
        }

        Ok(ResponseData {
            matched: true,
            result: json!({
                "Groups": groups,
            }),
        })
    } else {
        Ok(ResponseData {
            matched: false,
            result: json!({}),
        })
    }
}
