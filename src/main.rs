use bankex::{args::Args};
use std::io::Result;

fn main() -> Result<()> {
    let args = Args::parse();
    let Args {
        infile,
        verbose,
    } = args;

    if verbose == true {
        println!("argument parse result infile:[{}] verbose:[{}]",infile,verbose);
    }

   Ok(())
}
