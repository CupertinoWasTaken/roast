use std::env::Args;

use super::commands::Command;
// pub type CommandFn = dyn Fn(&[String]);

pub fn parse(mut arguments: Args) {
    arguments.next();
    let arguments: Vec<String> = arguments.collect();

    let command: Command = arguments.first().map_or("", |v| v).into();
    let subcommands = if arguments.len() > 1 {
        &arguments[1..]
    } else {
        &[]
    };

    println!("{command:?}\n{subcommands:?}");
}
