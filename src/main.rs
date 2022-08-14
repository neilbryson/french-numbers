mod convert;
mod words;

use crate::convert::convert;
use clap::Parser;

/// French number converter
#[derive(Parser)]
#[clap(name = "french-numbers")]
#[clap(about)]
#[clap(arg_required_else_help = true)]
struct Cli {
    /// Numbers to convert. To pass multiple numbers, input space-separated values.
    numbers: Vec<u32>,
}

const MAX_ALLOWED_NUMBER: u32 = 999999;

fn main() {
    let args: Cli = Cli::parse();

    for value in args.numbers {
        if value > MAX_ALLOWED_NUMBER {
            eprintln!("{} => ERR: Number beyond supported range", value);
            continue;
        }

        let in_words = convert(value);

        println!("{} => \"{}\"", value, in_words);
    }
}
