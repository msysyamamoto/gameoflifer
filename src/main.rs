use clap::Parser;
use gameoflifer::{run, Config};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None, disable_help_flag = true)]
struct Args {
    /// File name
    #[arg(short, long, default_value_t = String::from("-"))]
    filename: String,

    /// Character representing a living cell.
    #[arg(short, long, default_value_t = 'O')]
    character: char,

    /// Sleep milliseconds
    #[arg(short, long, default_value_t = 100)]
    sleepmillis: u64,

    /// Print help
    #[arg(long, action = clap::ArgAction::Help)]
    help: Option<bool>,
}

fn main() {
    let args = Args::parse();
    if let Err(e) = run(Config::new(args.filename, args.sleepmillis, args.character)) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
