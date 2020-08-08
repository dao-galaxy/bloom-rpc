
use jsonrpc_http_server::jsonrpc_core;

/// RPC methods metadata.
#[derive(Clone, Default, Debug)]
pub struct Metadata {
}

impl jsonrpc_core::Metadata for Metadata {}