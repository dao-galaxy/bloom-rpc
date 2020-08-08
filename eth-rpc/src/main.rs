mod rpc;

use jsonrpc_core::*;
use jsonrpc_http_server::{AccessControlAllowOrigin, DomainsValidation, RestApi, ServerBuilder};
use std::collections::HashMap;
use std::vec::Vec;
use serde_json;
use std::sync::atomic::{self, AtomicUsize};
use jsonrpc_core::futures::future::Either;
use jsonrpc_core::futures::Future;

use std::env;
use std::str::FromStr;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use rpc::Metadata;
use rpc::Eth;
use rpc::EthClient;
use rpc::types::Bytes;



#[derive(Default)]
struct MyMiddleware(AtomicUsize);
impl Middleware<Metadata> for MyMiddleware {
	type Future = FutureResponse;
	type CallFuture = middleware::NoopCallFuture;

	fn on_request<F, X>(&self, request: Request, meta: Metadata, next: F) -> Either<Self::Future, X>
		where
			F: FnOnce(Request, Metadata) -> X + Send,
			X: Future<Item = Option<Response>, Error = ()> + Send + 'static,
	{

		println!("\n==== ==== ==== ====\n");
		println!("Original Request Structure: {:?}", request);

		Either::A(Box::new(next(request, meta).map(move |res| {
			println!("Response Structure: {:?}", &res);
			res
		})))
	}
}


fn main() {
	let mut arguments = Vec::new();
	for arg in env::args() {
		arguments.push(arg);
	}

	let mut port  = 3030;
	if arguments.len() > 1 {
		port = i32::from_str(&arguments[1]).expect("Port argument is not invalid");
	}

	let mut io = MetaIoHandler::with_middleware(MyMiddleware::default());

	let client = EthClient::new().to_delegate();
	io.extend_with(client);

	let socket = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), port as u16);
	let server = ServerBuilder::new(io)
		.threads(3)
		.rest_api(RestApi::Unsecure)
		.cors(DomainsValidation::AllowOnly(vec![AccessControlAllowOrigin::Any]))
		.start_http(&socket)
		.expect("Unable to start RPC server");

	server.wait();
}
