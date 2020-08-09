
use crate::rpc::*;
use jsonrpc_core::{Result, BoxFuture};
use jsonrpc_derive::rpc;
use ethereum_types::{H64, H160, H256, U64, U256};



/// bloom rpc interface.
#[rpc(server)]
pub trait RpcMethod {
    /// RPC Metadata
    type Metadata;

    /// Returns accounts list.
    #[rpc(name = "accounts")]
    fn accounts(&self) -> Result<Vec<H160>>;

    /// Returns current gas_price.
    #[rpc(name = "gasPrice")]
    fn gas_price(&self) -> BoxFuture<U256>;

    /// Returns highest block number.
    #[rpc(name = "blockNumber")]
    fn block_number(&self) -> Result<U256>;

    /// Returns balance of the given account.
    #[rpc(name = "getBalance")]
    fn balance(&self, _: H160, _: Option<BlockNumber>) -> BoxFuture<U256>;

    /// Send transaction
    #[rpc(name = "sendTransaction")]
    fn send_transaction(&self, _: TransactionRequest) -> BoxFuture<H256>;

    /// Sends signed transaction, returning its hash.
    #[rpc(name = "sendRawTransaction")]
    fn send_raw_transaction(&self, _: Bytes) -> Result<H256>;

    /// Returns content of the storage at given address.
    #[rpc(name = "getStorageAt")]
    fn storage_at(&self, _: H160, _: U256, _: Option<BlockNumber>) -> BoxFuture<H256>;

    /// Returns the code at given address at given time (block number).
    #[rpc(name = "getCode")]
    fn code_at(&self, _: H160, _: Option<BlockNumber>) -> BoxFuture<Bytes>;

    /// Call contract, returning the output data.
    #[rpc(name = "call")]
    fn call(&self, _: CallRequest, _: Option<BlockNumber>) -> BoxFuture<Bytes>;
}