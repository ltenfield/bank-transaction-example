use crate::{AccountStatus, Ledger, Transaction, TransactionType};
use std::{collections::HashMap, error::Error};
use rust_decimal::Decimal;

pub struct InMemoryLedger {
    pub clients: HashMap<u16, AccountStatus>
}

impl InMemoryLedger {

    pub fn new() -> InMemoryLedger {
        return InMemoryLedger{ clients: HashMap::new() }
    }

    fn create_empty_accountstatus(&mut self, client_id: u16) -> AccountStatus {
        let nas = AccountStatus { 
            client: client_id, 
            available: Decimal::new(0,0), 
            held: Decimal::new(0,0), locked: false, 
            total: Decimal::new(0,0)};
        return nas;
    }

    pub fn process_deposit(&mut self, trans: &Transaction) -> Result<(), Box<dyn Error>> {
        println!("incomming transaction:[{:?}] clients:[{:?}]",trans,self.clients.len());
        let cid = match trans.client {
            Some(v) => v,
            None => return Err("need client id from transaction".into()),
        };
        //let uscid = cid as usize;
        //let old = self.clients.insert(cid, nas);
        let v = self.clients.get_mut(&cid);
        println!("initial account status:[{:?}]",v);
        if let Some(accs1) = v {
            println!("found account:[{:?}]",accs1);
            match trans.amount {
                Some(dec) => {
                    accs1.available = match accs1.available.checked_add(dec) {
                        Some(v3) => v3,
                        None => accs1.available,
                    };
                    println!("found [{:?}]  added:[{:?}]",accs1,dec);
                },
                None => {}
            };
        } else {
            { 
                let mut accs = AccountStatus { 
                    client: cid, 
                    available: Decimal::new(0,0), 
                    held: Decimal::new(0,0), locked: false, 
                    total: Decimal::new(0,0)};
                println!("creating account status adding trans:[{:?}]",trans);
                //let mut accs = self.create_empty_accountstatus(cid);
                match trans.amount {
                    Some(dec) => {
                        accs.available = match accs.available.checked_add(dec) {
                            Some(v2) => v2,
                            None => accs.available,
                        };
                        println!("available added:[{:?}] to get:[{:?}]",dec,accs);
                    },
                    None => {}
                };

            }
        }

        //let zero :u16 = 0;
        // let r = match v {
        //     Some(&as) => as.cid,
        //     None => zero
        // };
        //println!("got AccountStatus:[{:?}] old:[{:?}]",v,v);

        return Ok(());
        //let a_string_error = "not implemented".to_string();
        //let a_boxed_error = Box::<dyn Error>::from(a_string_error);
        //return Err("not implemented".into());
    }

    fn process_withdrawal(&mut self, trans: &Transaction) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    fn process_chargeback(&mut self, trans: &Transaction) -> Result<(), Box<dyn Error>> {
        let a_string_error = "not implemented".to_string();
        let a_boxed_error = Box::<dyn Error>::from(a_string_error);
        return Err(a_boxed_error);
    }

    fn process_dispute(&mut self, trans: &Transaction) -> Result<(), Box<dyn Error>> {
        let a_string_error = "not implemented".to_string();
        let a_boxed_error = Box::<dyn Error>::from(a_string_error);
        return Err(a_boxed_error);
    }

    fn process_resolve(&mut self, trans: &Transaction) -> Result<(), Box<dyn Error>> {
        let a_string_error = "not implemented".to_string();
        let a_boxed_error = Box::<dyn Error>::from(a_string_error);
        return Err(a_boxed_error);
    }
}

impl Ledger for InMemoryLedger {
    fn process_transaction(&mut self, trans: &Transaction) -> Result<(), Box<dyn Error>> {
        Self::verify_transaction(trans).unwrap();
        match trans.transaction_type {
            TransactionType::Deposit => {
                self.process_deposit(trans)?;
            },
            TransactionType::Withdrawal => {
                self.process_withdrawal(trans)?; 
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
        return Ok(());
    }

    fn get_funds_available(&mut self, client_id: u16) -> Result<Decimal, Box<dyn Error>> {
        let a_string_error = "not implemented".to_string();
        let a_boxed_error = Box::<dyn Error>::from(a_string_error);
        return Err(a_boxed_error);
    }
    
    fn get_funds_held(&mut self, client_id: u16) -> Result<Decimal, Box<dyn Error>> {
        let a_string_error = "not implemented".to_string();
        let a_boxed_error = Box::<dyn Error>::from(a_string_error);
        return Err(a_boxed_error);
    }
    
    fn get_funds_total(&mut self, client_id: u16) -> Result<Decimal, Box<dyn Error>> {
        let a_string_error = "not implemented".to_string();
        let a_boxed_error = Box::<dyn Error>::from(a_string_error);
        return Err(a_boxed_error);
    }
    
    fn get_all_client_ids(&mut self) -> Result<Vec<u16>, Box<dyn Error>> {
        let a_string_error = "not implemented".to_string();
        let a_boxed_error = Box::<dyn Error>::from(a_string_error);
        return Err(a_boxed_error);
    }

    fn verify_transaction(trans: &Transaction) -> Result<(), Box<dyn Error>> {
        if trans.client == None || trans.tx == None {
            return Err("Illegal state error".into());
        }
        if trans.transaction_type != TransactionType::Dispute || trans.transaction_type != TransactionType::Resolve {
            return Err("Illegal state error".into());
        }
        Ok(())
        // let a_string_error = "not implemented".to_string();
        // let a_boxed_error = Box::<dyn Error>::from(a_string_error);
        // return Err(a_boxed_error);
    }
    
}