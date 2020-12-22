use csv::Trim;
//use serde::Deserialize;
use crate::{MAX_DECIMAL_PLACES, Transaction};
use csv::{ReaderBuilder};
use std::error::Error;
use rust_decimal::Decimal;
use std::collections::{HashMap,hash_map::Entry::{Occupied,Vacant}};
use std::collections::BTreeMap;

//use std::{fs::File};

pub fn transaction_reader(verbose: bool, path: &str) -> Result<(BTreeMap<u32, Transaction>, Vec<Transaction>, Vec<Transaction>, Vec<Transaction>), Box<dyn Error>> {
    let mut rb = ReaderBuilder::new();
    let mut rdr = rb
        .flexible(true) // needed to allow optional amount column at end
        .trim(Trim::All)// needed to enable field parsing
        .from_path(path)?;
    let it = rdr.deserialize();
    let mut dispute_result: Vec<Transaction> = Vec::new();
    let mut resolve_result: Vec<Transaction> = Vec::new();
    let mut chargeback_result: Vec<Transaction> = Vec::new();
    let mut wd_result = BTreeMap::<u32, Transaction>::new();
    for record in it {
        let mut trans: Transaction = match record {
            Ok(t) => t,
            Err(e) => return Err(Box::new(e)),
        };
        let tid = match trans.tx {
            Some(v) => v,
            None => return Err("No transaction id".into())
        };
        let original_amount = match trans.amount {
            Some(a) => a,
            None => Decimal::new(0,0)
        };    
        if original_amount.scale() > MAX_DECIMAL_PLACES {
            let original_scale = original_amount.scale();
            let rounded_amount = original_amount.round_dp(MAX_DECIMAL_PLACES);
            trans.amount = Some(rounded_amount);
            if verbose {
                eprintln!("amount scale permitted exceeded max decimal places:[{:?}] will round :[original amount:[{:?}] original scale:[{:?}] new amount:[{:?}] new scale:[{:?}]]"
                    ,MAX_DECIMAL_PLACES,original_amount,original_scale,rounded_amount,rounded_amount.scale());
            }                          
        }
        if verbose {
            let verbose_amount = match trans.amount {
                Some(a) => a,
                None => Decimal::new(0,0)
            };    
            eprintln!("transaction:[{:?} amount scale:[{:?}]]",trans,verbose_amount.scale());
        }
        match trans.transaction_type {
            crate::TransactionType::Deposit => {
                wd_result.insert(tid, trans);
            },
            crate::TransactionType::Withdrawal => {
                wd_result.insert(tid, trans);
            }
            crate::TransactionType::Dispute => {
                dispute_result.push(trans);
            }
            crate::TransactionType::Resolve => {
                resolve_result.push(trans);
            }
            crate::TransactionType::Chargeback => {
                chargeback_result.push(trans);
            }
        }

        //wd_result.insert(tid, trans);
        // match result.entry(tid) {
        //     Occupied(_) => {
        //         if verbose {
        //             eprintln!("tx id:[{:?}] already exists skipping record",tid);
        //         }
        //     },
        //     Vacant(entry) => {
        //         entry.insert(trans);
        //     }
        // };
    }
    Ok((wd_result, dispute_result, resolve_result, chargeback_result))
}

// pub fn transaction_iterator(verbose: bool, path: &str) -> Result<DeserializeRecordsIter<File, Transaction>, Box<Error>> {
//     let mut rb = ReaderBuilder::new();
//     let mut rdr = rb.trim(Trim::All).from_path(path)?;
//     let it = rdr.deserialize();
//     return Ok(it);
// }