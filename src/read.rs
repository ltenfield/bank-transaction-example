use csv::Trim;
//use serde::Deserialize;
use crate::{MAX_DECIMAL_PLACES, Transaction};
use csv::{Error, ReaderBuilder};
use rust_decimal::Decimal;
//use std::{fs::File};

pub fn transaction_reader(verbose: bool, path: &str) -> Result<Vec<Transaction>, Box<Error>> {
    let mut rb = ReaderBuilder::new();
    let mut rdr = rb
        .flexible(true) // needed to allow optional amount column at end
        .trim(Trim::All)// needed to enable field parsing
        .from_path(path)?;
    let it = rdr.deserialize();
    let mut result: Vec<Transaction> = Vec::new();
    for record in it {
        let mut trans: Transaction = match record {
            Ok(t) => t,
            Err(e) => return Err(Box::new(e)),
                //Transaction{ transaction_type: crate::TransactionType::Deposit, client: None, tx: None, amount: None }
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
        result.push(trans);
    }
    Ok(result)
}

// pub fn transaction_iterator(verbose: bool, path: &str) -> Result<DeserializeRecordsIter<File, Transaction>, Box<Error>> {
//     let mut rb = ReaderBuilder::new();
//     let mut rdr = rb.trim(Trim::All).from_path(path)?;
//     let it = rdr.deserialize();
//     return Ok(it);
// }