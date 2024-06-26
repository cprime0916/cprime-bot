use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Config{
    pub discord: Discord,
    pub clist: Clist,
    pub tetr: Tetr,
}

#[derive(Deserialize, Debug)]
pub struct Discord{
    pub token: String,
    pub user_id: String,
}

#[derive(Deserialize, Debug)]
pub struct Clist{
    pub username: String,
    pub url: String,
    pub key: String,
}

#[derive(Deserialize, Debug)]
pub struct Tetr{
    pub general_url: String,
    pub user_url: String,
}
