use anyhow::{Context, Result};
use jsonrpc_http_server::*;
use jsonrpc_http_server::jsonrpc_core::*;
use serde_json::json;

pub fn new_server(address: &String, port: u16, allow_cors: bool) -> Result<Server> {
    let mut io = IoHandler::default();
    io.add_method("ping", |_: Params| {
	    Ok(Value::String("pong".into()))
    });
    io.add_method("tic", |_: Params| {
	    Ok(Value::String("tok".into()))
    });
    io.add_method("version", |params: Params| {
        let message = json!({
            "message": "OK",
            "params": params
        });
	    Ok(message)
    });

    let server = ServerBuilder::new(io)
        .cors(DomainsValidation::AllowOnly(vec![AccessControlAllowOrigin::Null]))
        .rest_api(RestApi::Secure)
        .start_http(&format!("{address}:{port}").parse()?)
        .context("Failed to launch rpc server")?;

    Ok(server)
}
