use crate::{AccountStatus, Ledger, Transaction, TransactionType};
use std::collections::{HashMap,hash_map::Entry::{Occupied,Vacant}};
use std::{error::Error};
use rust_decimal::Decimal;

const ILLEGAL_STATE: &'static str = "Illegal state error";
const NOT_IMPLEMENTED: &'static str = "not implemented";

#[derive(Debug)]
pub struct InMemoryLedger {
    pub by_client_id: HashMap<u16, AccountStatus>,
    pub by_transaction_id: HashMap<u32, AccountStatus>
}

impl Default for InMemoryLedger {
    fn default() -> Self {
        Self {
            by_client_id: HashMap::new(),
            by_transaction_id: HashMap::new()
        }
    }
}

impl InMemoryLedger {

    fn create_empty_accountstatus(client_id: u16) -> AccountStatus {
        let nas = AccountStatus { 
            client: client_id, 
            available: Decimal::new(0,0), 
            held: Decimal::new(0,0), locked: false, 
            total: Decimal::new(0,0)};
        return nas;
    }

    pub fn process_deposit(&mut self, trans: &Transaction) -> Result<(), Box<dyn Error>> {
        let cid = match trans.client {
            Some(v) => v,
            None => return Err("need client id from transaction".into()),
        };
        let tid = match trans.tx {
            Some(v) => v,
            None => return Err("need transaction id from transaction".into()),
        };
        let amount = match trans.amount {
            Some(v) => {v},
            None => {Decimal::new(0,0)},
        };
        match self.by_client_id.entry(cid) {
            Occupied(mut entry) => {
                let mut acct_status = entry.get_mut();
                let current_available = acct_status.available;
                acct_status.available = current_available + amount;
            },
            Vacant(entry) => {
                let mut acct_status = Self::create_empty_accountstatus(cid);
                acct_status.available = amount;
                entry.insert(acct_status);
            }
        }
        match self.by_transaction_id.entry(tid) {
            Occupied(mut entry) => {
                let mut acct_status = entry.get_mut();
                let current_available = acct_status.available;
                acct_status.available = current_available + amount;               
            },
            Vacant(entry) => {
                let mut acct_status = Self::create_empty_accountstatus(cid);
                acct_status.available = amount;
                entry.insert(acct_status);
            }
        }
        return Ok(());
    }

    pub fn process_withdrawal(&mut self,trans: &Transaction) -> Result<(), Box<dyn Error>> {
        let cid = match trans.client {
            Some(v) => v,
            None => return Err("need client id from transaction".into()),
        };
        let tid = match trans.tx {
            Some(v) => v,
            None => return Err("need transaction id from transaction".into()),
        };
        let amount = match trans.amount {
            Some(v) => {v},
            None => return Err("need amount from transaction".into()),
        };
        match self.by_client_id.entry(cid) {
            Occupied(mut entry) => {
                let mut acct_status = entry.get_mut();
                let current_available = acct_status.available;
                if amount > current_available {
                    return Err("Insufficient funds".into());
                }
                acct_status.available = current_available - amount;
            },
            Vacant(_) => {
                return Err("Insufficient funds, non existent by client id".into());
            }
        }
        match self.by_transaction_id.entry(tid) {
            Occupied(mut entry) => {
                let mut acct_status = entry.get_mut();
                let current_available = acct_status.available;
                if amount > current_available {
                    return Err("Insufficient funds".into());
                }
                acct_status.available = current_available - amount;
            },
            Vacant(_) => {
                return Err("Insufficient funds, non existent by transaction id".into());
            }
        }
        return Ok(());
    }

    fn process_chargeback(&self,trans: &Transaction) -> Result<(), Box<dyn Error>> {
        return Err(NOT_IMPLEMENTED.into());
    }

    fn process_dispute(&self,trans: &Transaction) -> Result<(), Box<dyn Error>> {
        return Err(NOT_IMPLEMENTED.into());
    }

    fn process_resolve(&self,trans: &Transaction) -> Result<(), Box<dyn Error>> {
        return Err(NOT_IMPLEMENTED.into());
    }
}

impl Ledger for InMemoryLedger {
    fn process_transaction(&mut self,verbose: bool, trans: &Transaction) -> Result<(), Box<dyn Error>> {
        if verbose {
            eprintln!("incomming transaction:[{:?}] available:[{:?}]",trans,self.by_client_id);
        }
        Self::verify_transaction(&self,trans).unwrap();
        match trans.transaction_type {
            TransactionType::Deposit => {
                self.process_deposit(trans)?;
            },
            TransactionType::Withdrawal => {
                match self.process_withdrawal(trans) {
                    Ok(_) => {},
                    Err(e) => {
                        if verbose {
                            eprintln!("skipping withdrawal transaction reason:[{}]",e);
                        }
                    }
                } 
            },
            TransactionType::Chargeback => {
                self.process_chargeback(trans)?; 
            },
            TransactionType::Dispute => {
                self.process_dispute(trans)?; 
            },
            TransactionType::Resolve => {
                self.process_resolve(trans)?; 
            }
        };
        if verbose {
            eprintln!("after transaction:[{:?}] available:[{:?}]",trans,self.by_client_id);
        }        
        return Ok(());
    }

    fn get_funds_available(client_id: u16) -> Result<Decimal, Box<dyn Error>> {
        return Err(NOT_IMPLEMENTED.into());    }
    
    fn get_funds_held(client_id: u16) -> Result<Decimal, Box<dyn Error>> {
        return Err(NOT_IMPLEMENTED.into());    }
    
    fn get_funds_total(client_id: u16) -> Result<Decimal, Box<dyn Error>> {
        return Err(NOT_IMPLEMENTED.into());    }
    
    fn get_all_client_ids() -> Result<Vec<u16>, Box<dyn Error>> {
        return Err(NOT_IMPLEMENTED.into());    }

    fn verify_transaction(&self, trans: &Transaction) -> Result<(), Box<dyn Error>> {
        if trans.client == None || trans.tx == None {
            return Err(ILLEGAL_STATE.into());
        }
        if trans.transaction_type == TransactionType::Dispute || trans.transaction_type == TransactionType::Resolve {
            if trans.amount != None {
                return Err(ILLEGAL_STATE.into());
            }
        } else {
            if trans.amount == None {
                return Err(ILLEGAL_STATE.into());
            }
        }
        Ok(())
    }
    
}