use crate::methods::*;
use crate::protocol::*;
use regex::Regex;

pub fn handle_request(req: RpcRequest) -> RpcResponse {
    if req.jsonrpc != "2.0" {
        return RpcResponse {
            jsonrpc: "2.0".into(),
            id: req.id,
            result: None,
            error: Some(RpcError {
                code: -32600,
                message: "invalid jsonrpc version".into(),
            }),
        };
    }

    let regex = match Regex::new(&req.params.pattern) {
        Ok(re) => Some(re),

        Err(err) => {
            return RpcResponse {
                jsonrpc: "2.0".into(),
                id: req.id,
                result: None,
                error: Some(RpcError {
                    code: -32001,
                    message: err.to_string(),
                }),
            };
        }
    };

    let result = match req.method.as_str() {
        "find" => find_method::handle(regex.as_ref().unwrap(), &req.params),

        "find_iter" => find_iter_method::handle(regex.as_ref().unwrap(), &req.params),

        "captures" => captures_method::handle(regex.as_ref().unwrap(), &req.params),

        "replace" => replace_method::handle(regex.as_ref().unwrap(), &req.params),

        "replace_all" => replace_all_method::handle(regex.as_ref().unwrap(), &req.params),

        "split" => split_method::handle(regex.as_ref().unwrap(), &req.params),

        "escape" => escape_method::handle(&req.params),

        "is_match" => is_match_method::handle(regex.as_ref().unwrap(), &req.params),

        _ => {
            return RpcResponse {
                jsonrpc: "2.0".into(),
                id: req.id,
                result: None,
                error: Some(RpcError {
                    code: -32601,
                    message: "method not found".into(),
                }),
            };
        }
    };

    match result {
        Ok(v) => RpcResponse {
            jsonrpc: "2.0".into(),
            id: req.id,
            result: Some(v),
            error: None,
        },

        Err(err) => RpcResponse {
            jsonrpc: "2.0".into(),
            id: req.id,
            result: None,
            error: Some(RpcError {
                code: -32000,
                message: err.to_string(),
            }),
        },
    }
}
