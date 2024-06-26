use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config{
    pub discord: Discord,
    pub clist: Clist,
    pub tetr: Tetr,
}

#[derive(Deserialize)]
pub struct Discord{
    pub token: String,
    pub user_id: String,
}

#[derive(Deserialize)]
pub struct Clist{
    pub username: String,
    pub url: String,
    pub key: String,
}

#[derive(Deserialize)]
pub struct Tetr{
    pub general_url: String,
    pub user_url: String,
}
