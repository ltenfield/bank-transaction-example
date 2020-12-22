use rust_decimal::Decimal;
use serde::Deserialize;
use std::error::Error;
pub mod args;
pub mod read;
pub mod ledger;

const MAX_DECIMAL_PLACES: u32 = 4;

#[derive(Debug, Deserialize, PartialEq)]
enum TransactionType {
   #[serde(rename = "deposit")]
   Deposit,
   #[serde(rename = "withdrawal")]
   Withdrawal,
   #[serde(rename = "dispute")]
   Dispute,
   #[serde(rename = "resolve")]
   Resolve,
   #[serde(rename = "chargeback")]
   Chargeback
}

#[derive(Debug, Deserialize)]
pub struct Transaction {
   #[serde(rename = "type")]
   transaction_type: TransactionType,
   client: Option<u16>,
   tx: Option<u32>,
   amount: Option<Decimal>
}
#[derive(Debug, Clone)]
pub struct AccountStatus {
   client:   u16,
   available:    Decimal,
   held:    Decimal,
   total:    Decimal,
   locked:  bool
}

pub trait Ledger {
   fn process_transaction(&mut self,verbose: bool, trans: &Transaction) -> Result<(), Box<dyn Error>>;
   fn get_funds_available(client_id: u16) -> Result<Decimal, Box<dyn Error>>;
   fn get_funds_held( client_id: u16) -> Result<Decimal, Box<dyn Error>>;
   fn get_funds_total( client_id: u16) -> Result<Decimal, Box<dyn Error>>;
   fn get_all_client_ids() -> Result<Vec<u16>, Box<dyn Error>>;
   fn verify_transaction(&self, trans: &Transaction) -> Result<(), Box<dyn Error>>;
}