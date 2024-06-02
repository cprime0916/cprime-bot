use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config{
    pub discord: Discord,
}

#[derive(Deserialize)]
pub struct Discord{
    pub token: String,
}