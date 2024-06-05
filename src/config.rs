use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config{
    pub discord: Discord,
    pub clist: Clist,
}

#[derive(Deserialize)]
pub struct Discord{
    pub token: String,
}

#[derive(Deserialize)]
pub struct Clist{
    pub username: String,
    pub url: String,
    pub key: String,
}