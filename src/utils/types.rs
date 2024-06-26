#[derive(Debug)]
pub(crate) enum FileType {
    Json,
    Toml,
    Unknown,
}



impl From<FileType> for String {
    fn from(value: FileType) -> Self {
        match value{
            FileType::Json => String::from("json"),
            FileType::Toml => String::from("toml"),
            FileType::Unknown => String::from("unknown")
        }
    }
}

#[derive(Debug)]
pub(crate) enum ExpectError {
    ReactionNotFound,
    ParseError,
    Unknown,
}

impl From<ExpectError> for String{
    fn from(value: ExpectError) -> Self {
        match value {
            ExpectError::ReactionNotFound => String::from("Reaction not found"),
            ExpectError::ParseError => String::from("Parsing error"),
            ExpectError::Unknown => String::from("Unknown error"),
        }
    }
}