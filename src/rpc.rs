use crate::handler::handle_request;
use crate::protocol::*;
use anyhow::Result;
use std::io::{self, BufRead, Write};

pub fn run() -> Result<()> {
    let stdin = io::stdin();
    let mut stdout = io::stdout();

    for line in stdin.lock().lines() {
        let line = line?;

        if line.trim().is_empty() {
            continue;
        }

        let req: RpcRequest = match serde_json::from_str(&line) {
            Ok(v) => v,
            Err(err) => {
                let resp = RpcResponse {
                    jsonrpc: "2.0".into(),
                    id: serde_json::Value::Null,
                    result: None,
                    error: Some(RpcError {
                        code: -32700,
                        message: err.to_string(),
                    }),
                };

                writeln!(stdout, "{}", serde_json::to_string(&resp)?)?;

                stdout.flush()?;

                continue;
            }
        };

        let resp = handle_request(req);

        writeln!(stdout, "{}", serde_json::to_string(&resp)?)?;

        stdout.flush()?;
    }

    Ok(())
}
