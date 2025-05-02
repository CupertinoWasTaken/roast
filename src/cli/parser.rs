use std::collections::HashMap;
use std::env::Args;
use std::sync::LazyLock;

use super::commands::{self, Command};

type CommandFn = fn(&[String]);

static COMMAND_MAP: LazyLock<HashMap<Command, CommandFn>> =
    LazyLock::new(|| HashMap::from([(Command::Help, commands::help as CommandFn)]));

pub fn parse(mut arguments: Args) {
    arguments.next();
    let arguments: Vec<String> = arguments.collect();

    let command: Command = arguments.first().map_or("", |v| v).into();
    let subcommands = if arguments.len() > 1 {
        &arguments[1..]
    } else {
        &[]
    };

    if let Some(command_fn) = COMMAND_MAP.get(&command) {
        command_fn(subcommands)
    } else {
        println!("Command is reserved, but not implemented yet.")
    }
}
