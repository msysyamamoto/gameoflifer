use clap::Parser;
#[derive(Parser, Debug)]
#[command(version, about, long_about = None, disable_help_flag = true)]
struct Args {
    /// Sleep milliseconds
    #[arg(short, long, default_value_t = 100)]
    sleepmillis: u64,

    /// File name
    #[arg(short, long, default_value_t = String::from("-"))]
    filename: String,

    /// Print help
    #[arg(long, action = clap::ArgAction::Help)]
    help: Option<bool>,
}

fn main() {
    let args = Args::parse();
    if let Err(e) = gameoflifer::run(&args.filename) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
