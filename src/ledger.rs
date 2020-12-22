use crate::{AccountStatus, Ledger, Transaction, TransactionType};
use std::collections::{HashMap,hash_map::Entry::{Occupied,Vacant}};
use std::{error::Error};
use rust_decimal::Decimal;

const ILLEGAL_STATE: &'static str = "Illegal state error";
const NOT_IMPLEMENTED: &'static str = "not implemented";

#[derive(Debug)]
pub struct InMemoryLedger {
    pub by_client_id: HashMap<u16, AccountStatus>,
    pub by_transaction_id: HashMap<u32, Transaction>
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
        match self.by_transaction_id.entry(tid) {
            Occupied(_) => {
                //let bad_trans = entry.get();
                return Err("Duplicate transaction".into());
            },
            Vacant(entry) => {
                entry.insert(trans.clone());
            }
        }
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
        match self.by_transaction_id.entry(tid) {
            Occupied(_) => {
                //let bad_trans = entry.get();
                return Err("Duplicate transaction".into());
            },
            Vacant(entry) => {
                entry.insert(trans.clone());
            }
        }
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
        return Ok(());
    }

    fn process_dispute(&mut self, verbose: bool, trans: &Transaction) -> Result<(), Box<dyn Error>> {
        let cid = match trans.client {
            Some(v) => v,
            None => return Err("need client id from transaction".into()),
        };
        let tid = match trans.tx {
            Some(v) => v,
            None => return Err("need transaction id from transaction".into()),
        };
        match self.by_transaction_id.entry(tid) {
            Occupied(client_transaction) => {
                let ct = client_transaction.get();
                let clientid = match ct.client {
                    Some(v) => v,
                    None => 0
                };
                if clientid == cid {  // only proceed if transaction is for the right client id indicated in dispute
                    match self.by_client_id.entry(cid) {
                        Occupied(mut client_account_status) => {
                            let mut cas = client_account_status.get_mut();
                            let cat_amount_val = match ct.amount {
                                Some(v) => v,
                                None => Decimal::new(0,0),
                            };
                            if cat_amount_val <= cas.available {
                                cas.available = cas.available - cat_amount_val;
                                cas.held = cas.held + cat_amount_val;
                                if verbose {
                                    eprintln!("DISPUTE: Funds:[{:?}] held for client id:[{:?}]",cat_amount_val,cid);
                                }
                            }
                        },
                        Vacant(_) => {
                            if verbose {
                                eprintln!("DISPUTE: account status client id:[{:?}] not found",cid);
                            }
                        }
                    }
                } else {
                    if verbose {
                        eprintln!("DISPUTE: found transaction id:[{:?}] however not for client id:[{:?}]",tid,cid);
                    }                    
                }

            }
            Vacant(_) => {
                if verbose {
                    eprintln!("DISPUTE: transaction id:[{:?}] for client id:[{:?}] not found",tid,cid);
                }
            }
        }
        return Ok(());
    }

    fn process_resolve(&mut self,verbose: bool,trans: &Transaction) -> Result<(), Box<dyn Error>> {
        let cid = match trans.client {
            Some(v) => v,
            None => return Err("need client id from transaction".into()),
        };
        let tid = match trans.tx {
            Some(v) => v,
            None => return Err("need transaction id from transaction".into()),
        };
        match self.by_transaction_id.entry(tid) {
            Occupied(client_transaction) => {
                let ct = client_transaction.get();
                let clientid = match ct.client {
                    Some(v) => v,
                    None => 0
                };
                if clientid == cid {  // only proceed if transaction is for the right client id indicated in dispute
                    match self.by_client_id.entry(cid) {
                        Occupied(mut client_account_status) => {
                            let mut cas = client_account_status.get_mut();
                            let cat_amount_val = match ct.amount {
                                Some(v) => v,
                                None => Decimal::new(0,0),
                            };
                            if cat_amount_val <= cas.held { // must have enough in help to resolve amount
                                cas.available = cas.available + cat_amount_val;
                                cas.held = cas.held - cat_amount_val;
                                if verbose {
                                    eprintln!("RESOLVE: funds:[{:?}] held for client id:[{:?}] were returned",cat_amount_val,cid);
                                }
                            }
                        },
                        Vacant(_) => {
                            if verbose {
                                eprintln!("RESOLVE: account status client id:[{:?}] not found",cid);
                            }
                        }
                    }
                } else {
                    if verbose {
                        eprintln!("RESOLVE: found transaction id:[{:?}] however not for client id:[{:?}]",tid,cid);
                    }                    
                }

            }
            Vacant(_) => {
                if verbose {
                    eprintln!("RESOLVE: transaction id:[{:?}] for client id:[{:?}] not found",tid,cid);
                }
            }
        }
        return Ok(());
    }

    fn process_chargeback(&mut self,verbose: bool, trans: &Transaction) -> Result<(), Box<dyn Error>> {
        let cid = match trans.client {
            Some(v) => v,
            None => return Err("need client id from transaction".into()),
        };
        let tid = match trans.tx {
            Some(v) => v,
            None => return Err("need transaction id from transaction".into()),
        };
        match self.by_transaction_id.entry(tid) {
            Occupied(client_transaction) => {
                let ct = client_transaction.get();
                let clientid = match ct.client {
                    Some(v) => v,
                    None => 0
                };
                if clientid == cid {  // only proceed if transaction is for the right client id indicated in dispute
                    match self.by_client_id.entry(cid) {
                        Occupied(mut client_account_status) => {
                            let mut cas = client_account_status.get_mut();
                            let cat_amount_val = match ct.amount {
                                Some(v) => v,
                                None => Decimal::new(0,0),
                            };
                            if cat_amount_val <= cas.available { // must have enough in available to chargeback amount (i.e. withdraw from account)
                                cas.available = cas.available - cat_amount_val;
                                cas.locked = true; // always freeze account after chargeback 
                                if verbose {
                                    eprintln!("CHARGEBACK: funds:[{:?}] withdrawn for client id:[{:?}]",cat_amount_val,cid);
                                }
                            }
                        },
                        Vacant(_) => {
                            if verbose {
                                eprintln!("CHARGEBACK: account status client id:[{:?}] not found",cid);
                            }
                        }
                    }
                } else {
                    if verbose {
                        eprintln!("CHARGEBACK: found transaction id:[{:?}] however not for client id:[{:?}]",tid,cid);
                    }                    
                }

            }
            Vacant(_) => {
                if verbose {
                    eprintln!("CHARGEBACK: transaction id:[{:?}] for client id:[{:?}] not found",tid,cid);
                }
            }
        }
        return Ok(());
    }
}

impl Ledger for InMemoryLedger {
    fn process_transaction(&mut self,verbose: bool, trans: &Transaction) -> Result<(), Box<dyn Error>> {
        let cid = match trans.client {
            Some(v) => v,
            None => return Err("need client id from transaction".into()),
        };
        let account_status = self.by_client_id.get(&cid);
        if verbose {
            eprintln!("incomming transaction:[{:?}] available:[{:?}]",trans,account_status);
        }
        match account_status {
            Some(cas) => {
                if cas.locked {
                    if verbose {
                        eprintln!("client account:[{:?}] locked skipping transaction",cid);
                    }
                    return Ok(()); 
                }
            },
            None => {} // do nothing if account status for client is not found since account status could be created with transaction processing
        };
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
            TransactionType::Dispute => {
                self.process_dispute(verbose,trans)?; 
            },
            TransactionType::Resolve => {
                self.process_resolve(verbose,trans)?; 
            }
            TransactionType::Chargeback => {
                self.process_chargeback(verbose,trans)?; 
            },
        };
        let new_account_status = self.by_client_id.get(&cid);
        if verbose {
            eprintln!("after transaction:[{:?}] available:[{:?}]",trans,new_account_status);
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
        if trans.transaction_type == TransactionType::Dispute
            || trans.transaction_type == TransactionType::Resolve
            || trans.transaction_type == TransactionType::Chargeback {
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