use bankex::{Ledger, ledger::InMemoryLedger, read};

static VERBOSE: bool = false;
static ORIGINAL_EXAMPLE: &str = "type, client, tx, amount
deposit, 1, 1, 1.0
deposit, 2, 2, 2.0
deposit, 1, 3, 2.0
withdrawal, 1, 4, 1.5
withdrawal, 2, 5, 3.0";

// simply run original example without errors
#[test]
fn read_original_example() {
    let rdr = stringreader::StringReader::new(ORIGINAL_EXAMPLE);
    let (withdraw_deposits,
         disputes,
          resolves,
           chargebacks) = read::transaction_reader_from(VERBOSE, Box::new(rdr)).unwrap();
        let mut ledger = InMemoryLedger::default();
        ledger.read_transactions(false, withdraw_deposits, disputes, resolves, chargebacks);
        ledger.run_report();
}
