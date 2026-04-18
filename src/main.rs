mod cli;

fn main() {
    if let Err(e) = cli::parse() {
        eprintln!("{e}");
        std::process::exit(1);
    }
}
