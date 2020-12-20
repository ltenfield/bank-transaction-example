use bankex::{args::Args,read};
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

    //Ok(())
}
