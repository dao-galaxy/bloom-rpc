
use crate::rpc::*;

use jsonrpc_core::{Result, BoxFuture};
use jsonrpc_core::futures::future;

use ethereum_types::{H64, H160, H256, U64, U256};


pub struct EthClient {}

impl EthClient {
    pub fn new() -> Self {
        EthClient{}
    }
}

impl Eth for EthClient {
    type Metadata = Metadata;

    fn accounts(&self) -> Result<Vec<H160>> {
        Ok(Vec::new())
    }

    fn gas_price(&self) -> BoxFuture<U256> {
        let bal = U256::zero();
        Box::new(future::done(Ok(bal)))
    }

    fn block_number(&self) -> Result<U256>{
        let bal = U256::zero();
        Ok(bal)
    }


    fn balance(&self, address: H160, num: Option<BlockNumber>) -> BoxFuture<U256> {
        let bal = U256::zero();
        Box::new(future::done(Ok(bal)))
    }

    fn send_transaction(&self, tx: TransactionRequest) -> BoxFuture<H256> {
        println!("{}",tx);
        Box::new(future::done(Ok(H256::default())))
    }

    fn send_raw_transaction(&self, tx: Bytes) -> Result<H256> {
        println!("send_raw_transaction:{:?}",tx);
        Ok(H256::default())
    }

    fn storage_at(&self, _: H160, _: U256, _: Option<BlockNumber>) -> BoxFuture<H256>{
        Box::new(future::done(Ok(H256::default())))
    }

    fn code_at(&self, _: H160, _: Option<BlockNumber>) -> BoxFuture<Bytes>{
        Box::new(future::done(Ok(Bytes::default())))
    }

    fn call(&self, _: CallRequest, _: Option<BlockNumber>) -> BoxFuture<Bytes>{
        Box::new(future::done(Ok(Bytes::default())))
    }

}