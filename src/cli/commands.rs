mod help;
pub use help::help;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Command {
    Add,
    Remove,
    Pour,
    Help,
}

impl From<&str> for Command {
    fn from(value: &str) -> Self {
        match value.to_lowercase().as_str() {
            "add" => Self::Add,
            "rem" | "rm" | "remove" => Self::Remove,
            "pour" | "build" => Self::Pour,
            _ => Self::Help,
        }
    }
}
