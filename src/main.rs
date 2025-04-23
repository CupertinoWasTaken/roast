mod cli;

fn main() {
    cli::parser::parse(std::env::args());
}