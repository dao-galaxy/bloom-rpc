
use crate::rpc::Bytes;
use ethereum_types::{H160,H256,U256};
use ansi_term::Colour;
use serde_derive::{Serialize, Deserialize};

use std::fmt;

#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransactionRequest {
    /// Sender
    pub from: Option<H160>,
    /// Recipient
    pub to: Option<H160>,
    /// Gas Price
    pub gas_price: Option<U256>,
    /// Gas
    pub gas: Option<U256>,
    /// Value of transaction in wei
    pub value: Option<U256>,
    /// Additional data sent with transaction
    pub data: Option<Bytes>
}

pub fn format_ether(i: U256) -> String {
    let mut string = format!("{}", i);
    let idx = string.len() as isize - 18;
    if idx <= 0 {
        let mut prefix = String::from("0.");
        for _ in 0..idx.abs() {
            prefix.push('0');
        }
        string = prefix + &string;
    } else {
        string.insert(idx as usize, '.');
    }
    String::from(string.trim_end_matches('0').trim_end_matches('.'))
}

impl fmt::Display for TransactionRequest {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let eth = self.value.unwrap_or_default();
        match self.to {
            Some(ref to) => write!(
                f,
                "{} ETH from {} to 0x{:?}",
                Colour::White.bold().paint(format_ether(eth)),
                Colour::White.bold().paint(
                    self.from.as_ref()
                        .map(|f| format!("0x{:?}", f))
                        .unwrap_or_else(|| "?".to_string())),
                to
            ),
            None => write!(
                f,
                "{} ETH from {} for contract creation",
                Colour::White.bold().paint(format_ether(eth)),
                Colour::White.bold().paint(
                    self.from.as_ref()
                        .map(|f| format!("0x{:?}", f))
                        .unwrap_or_else(|| "?".to_string())),
            ),
        }
    }
}





