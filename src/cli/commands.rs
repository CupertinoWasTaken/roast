#[derive(Debug)]
pub enum Command {
    Add,
    Remove,
    Pour,
    Help
}

impl From<&str> for Command {
    fn from(value: &str) -> Self {
        match value.to_lowercase().as_str() {
            "add" => Self::Add,
            "rem" | "remove" => Self::Remove,
            "pour" | "build" => Self::Pour,
            _ => Self::Help
        }
    }
}