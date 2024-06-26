use serde::Deserialize;
use crate::walao;

#[derive(Deserialize)]
pub struct ContestInfo {
    pub _meta: Option<Meta>,
    pub objects: Option<Vec<Contest>>,
}

#[derive(Deserialize)]
pub struct Meta {
    pub limit: Option<usize>,
    pub next: Option<String>,
    pub offset: Option<usize>,
    pub previous: Option<String>,
    pub total_count: Option<usize>,
}

#[derive(Deserialize)]
pub struct Contest {
    pub id: Option<usize>,
    pub resource: Option<String>,
    pub resource_id: Option<usize>,
    pub host: Option<String>,
    pub event: Option<String>,
    pub start: Option<String>,
    pub end: Option<String>,
    pub n_statistics: Option<usize>,
    pub n_problems: Option<usize>,
    pub parsed_at: Option<String>,
    pub duration: Option<usize>,
    pub href: Option<String>,
    pub problems: Option<String>,
}

#[derive(Deserialize)]
pub struct TetrInfo {
    pub success: bool,
    pub error: Option<String>,
    pub cache: Option<Cache>,
    pub data: Option<TetrUserData>,
}

impl TetrInfo {
    fn result(&self) -> Result<bool, String> {
        if let Some(s) = &self.error {
            return Err(s.to_owned());
        }
        Ok(self.success)
    }

    fn user(&self) -> Option<TetrUser> {
        if let Some(d) = &self.data {
            return Some(d.user.clone());
        }
        None
    }
}

#[derive(Deserialize)]
pub struct Cache {
    status: Option<String>,
    cached_at: isize,
    cached_until: isize,
}

#[derive(Deserialize)]
pub struct TetrUserData {
    user: TetrUser,
}

#[derive(Deserialize, Clone)]
pub struct TetrUser {
    _id: Option<String>,
    username: Option<String>,
    role: Option<String>,
    ts: Option<String>,
    botmaster: Option<String>,
}

impl TetrUser {
    fn username(&self) -> String {
        self.username
            .clone()
            .unwrap_or(walao!(unwrap, parse))
    }
    fn role(&self) -> String {
        self.role
            .clone()
            .unwrap_or(walao!(unwrap, parse))
    }
    fn timestamp(&self) -> String {
        self.ts
            .clone()
            .unwrap_or(walao!(unwrap, parse))
    }
    fn bot_info(&self) -> Option<String> {
        if let Some(s) = self.botmaster.clone() {
            return Some(s);
        }
        None
    }
}