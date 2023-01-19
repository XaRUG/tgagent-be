use anyhow::{Context, Result};
use jsonrpc_http_server::*;
use jsonrpc_http_server::jsonrpc_core::*;

pub fn new_server(address: &String, port: u16, allow_cors: bool) -> Result<Server> {
    let mut io = IoHandler::default();
	io.add_method("ping", |_: Params| {
	    Ok(Value::String("pong".into()))
	});

    let server = ServerBuilder::new(io)
        .cors(DomainsValidation::AllowOnly(vec![AccessControlAllowOrigin::Null]))
        .start_http(&format!("{address}:{port}").parse()?)
        .context("Failed to launch rpc server")?;

    Ok(server)
}
