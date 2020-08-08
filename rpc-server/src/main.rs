
#![feature(str_strip)]

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
use rpc::RpcMethod;
use rpc::BloomClient;
use rpc::types::Bytes;
use jsonrpc_core::types::request::Request::Single;
use jsonrpc_core::types::request::Call::MethodCall;
use jsonrpc_core::types::id::Id;
use jsonrpc_core::types::params::Params;
use jsonrpc_core::Output::Success;
use jsonrpc_core::types::response::Response::Single as RepSingle;
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
		let mut req = request;
		let mut obj = jsonrpc_core::types::request::MethodCall {
			jsonrpc: None,
			method: Default::default(),
			params: Params::None,
			id: Id::Null,
		};

		if let Single(MethodCall(mut mc)) = req {
			println!("Original Method: {:?}", mc.method);
			if mc.method.starts_with("eth_") {
				mc.method.replace_range(..4, "");
				println!("Modified Method: {:?}", mc.method);
			}
			obj = mc;
		}
		println!("Pre-Processed Request Object: {:?}", obj);

		Either::A(Box::new(next(Single(MethodCall(obj)), meta).map(move |res| {
			println!("Response Structure: {:?}", &res);
			if let Some(RepSingle(Success(res_obj))) = &res {
				println!("Response Object: {:?}", res_obj);
			}
			res
		})))
	}
}


fn main() {
	let args: Vec<_> = std::env::args().collect();

	let mut port  = 3030;
	if args.len() > 1 {
		port = i32::from_str(&args[1]).expect("Port argument is not invalid");
	}

	let mut io = MetaIoHandler::with_middleware(MyMiddleware::default());

	let client = BloomClient::new().to_delegate();
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

