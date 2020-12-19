use clap::{App, Arg};

pub struct Args {
    pub infile: String,
    pub verbose: bool,
}

impl Args {
    pub fn parse() -> Self {
        let matches = App::new("bankex")
            .version("0.1.0")
            .arg(Arg::with_name("infile")
                .takes_value(true).required(true).help("Read from a file instead of stdin"))
            .arg(Arg::with_name("verbose").short("v").long("verbose").help("debug and error output"))
            .get_matches();
        let infile = matches.value_of("infile").unwrap_or_default().to_string();
        let verbose = matches.is_present("verbose");
        Self {
            infile,
            verbose,
        }
    }
}
