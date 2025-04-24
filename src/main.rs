#![allow(unused)]
mod cli;
mod colors;

fn main() {
    cli::parser::parse(std::env::args());
}
