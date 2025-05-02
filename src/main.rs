#![allow(unused)]
mod bean;
mod cli;
mod colors;
mod triple;

fn main() {
    cli::parser::parse(std::env::args());
}
