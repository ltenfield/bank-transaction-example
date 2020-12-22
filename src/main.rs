use bankex::{Ledger, args::Args, ledger::InMemoryLedger, read};


fn main() {
    let args = Args::parse();
    let Args {
        infile,
        verbose,
    } = args;

    if verbose == true {
        eprintln!("argument parse result infile:[{}] verbose:[{}]",infile,verbose);
    }
    let (withdraw_deposits, disputes, resolves, chargebacks) = read::transaction_reader(verbose, &infile).unwrap();
    if verbose {
        eprintln!("got [{:?}] withdraw or deposit transactions",withdraw_deposits.len());
    }
    let mut ledger = InMemoryLedger::default();
    //let mut hm = ledger.clients;

    // ledger.process_transaction(&transactions[0]).unwrap();
    // ledger.process_transaction(&transactions[2]).unwrap();

    for (txid, transaction) in withdraw_deposits {
        ledger.process_transaction(verbose, &transaction).unwrap();
        if verbose {
            println!("txid:[{:?}] transaction:[{:?}]",txid,transaction);
        }
    }
    for transaction in disputes {
       ledger.process_transaction(verbose, &transaction).unwrap();
        if verbose {
            let txid = match transaction.tx {
                Some(v) => v,
                None => 0
            };
            println!("txid:[{:?}] transaction:[{:?}]",txid,transaction);
        }
    }
    for transaction in resolves {
       ledger.process_transaction(verbose, &transaction).unwrap();
       if verbose {
            let txid = match transaction.tx {
                Some(v) => v,
                None => 0
            };
            println!("txid:[{:?}] transaction:[{:?}]",txid,transaction);
        }
    }
    for transaction in chargebacks {
       ledger.process_transaction(verbose, &transaction).unwrap();
       if verbose {
            let txid = match transaction.tx {
                Some(v) => v,
                None => 0
            };
            println!("txid:[{:?}] transaction:[{:?}]",txid,transaction);
        }
    }
}
