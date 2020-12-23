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

    ledger.read_transactions(verbose, withdraw_deposits, disputes, resolves, chargebacks);

    ledger.run_report();
}
