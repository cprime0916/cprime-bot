#![allow(dead_code)]
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

#[derive(poise::ChoiceParameter)]
pub enum Algorithms{
    #[name="Depth-first search"]
    #[name_localized("zh-TW", "深度優先搜尋")]
    Dfs,
    #[name="Fibonacci memoization"]
    #[name_localized("zh-TW", "記憶化費波那契數")]
    DpFib,
    #[name="Binary search"]
    #[name_localized("zh-TW", "二分搜尋演算法")]
    BinSearch,
}