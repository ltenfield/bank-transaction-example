use rust_decimal::Decimal;
use serde::Deserialize;
use std::error::Error;
pub mod args;
pub mod read;
pub mod ledger;

const MAX_DECIMAL_PLACES: u32 = 4;

#[derive(Debug, Deserialize, PartialEq, Clone)]
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

#[derive(Debug, Deserialize, Clone)]
pub struct Transaction {
   #[serde(rename = "type")]
   transaction_type: TransactionType,
   client: Option<u16>,
   pub tx: Option<u32>,
   amount: Option<Decimal>
}
#[derive(Debug, Clone)]
pub struct AccountStatus {
   client:   u16,
   pub available:    Decimal,
   pub held:    Decimal,
   total:    Decimal,
   pub locked:  bool
}

pub trait Ledger {
   fn process_transaction(&mut self,verbose: bool, trans: &Transaction) -> Result<(), Box<dyn Error>>;
   fn get_funds_available(&self, client_id: u16) -> Result<Decimal, Box<dyn Error>>;
   fn get_funds_held(&self, client_id: u16) -> Result<Decimal, Box<dyn Error>>;
   fn get_funds_total(&self, client_id: u16) -> Result<Decimal, Box<dyn Error>>;
   fn verify_transaction(&self, trans: &Transaction) -> Result<(), Box<dyn Error>>;
}