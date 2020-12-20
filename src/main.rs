use bankex::{Ledger, args::Args, ledger::InMemoryLedger, read};
//use std::io::Result;

fn main() {
    let args = Args::parse();
    let Args {
        infile,
        verbose,
    } = args;

    if verbose == true {
        eprintln!("argument parse result infile:[{}] verbose:[{}]",infile,verbose);
    }
    let transactions = read::transaction_reader(verbose, &infile).unwrap();
    if verbose {
        eprintln!("got [{:?}] transactions",transactions.len());
    }
    let mut ledger = InMemoryLedger::new();
    //let mut hm = ledger.clients;

    ledger.process_deposit(&transactions[0]).unwrap();
    ledger.process_deposit(&transactions[2]).unwrap();

    //Ok(())
}
