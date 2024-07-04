#![allow(dead_code)]

use serde::Deserialize;
use crate::utils::types::ExpectError;
use crate::walao;
use std::fmt;

#[derive(Deserialize, Debug)]
pub struct ContestInfo {
    pub _meta: Option<Meta>,
    pub objects: Option<Vec<Contest>>,
}

#[derive(Deserialize, Debug)]
pub struct Meta {
    pub limit: Option<usize>,
    pub next: Option<String>,
    pub offset: Option<usize>,
    pub previous: Option<String>,
    pub total_count: Option<usize>,
}

#[derive(Deserialize, Debug)]
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

#[derive(Deserialize, Debug)]
pub struct TetrInfo {
    pub success: bool,
    pub error: Option<String>,
    pub cache: Option<Cache>,
    pub data: Option<TetrUserData>,
}

impl TetrInfo {
    pub fn result(&self) -> Result<bool, String> {
        if let Some(s) = &self.error {
            return Err(s.to_owned());
        }
        Ok(self.success)
    }

    pub fn user(&self) -> Option<TetrUser> {
        if let Some(d) = &self.data {
            return Some(d.user.clone());
        }
        None
    }
}

#[derive(Deserialize, Debug)]
pub struct Cache {
    status: Option<String>,
    cached_at: isize,
    cached_until: isize,
}

#[derive(Deserialize, Debug)]
pub struct TetrUserData {
    pub user: TetrUser,
}

#[derive(Deserialize, Clone, Debug)]
pub struct TetrUser {
    pub id: Option<String>,
    pub username: Option<String>,
    pub role: Option<String>,
    pub ts: Option<String>,
    pub botmaster: Option<String>,
    pub badges: Vec<Option<Badge>>,
    pub xp: Option<f64>,
    pub gamesplayed: Option<isize>,
    pub gameswon: Option<isize>,
    pub gametime: Option<f64>,
    pub country: Option<String>,
    pub badstanding: Option<bool>,
    pub supporter: Option<bool>,
    pub supporter_tier: Option<usize>,
    pub verified: Option<bool>,
    pub league: Option<League>,
    pub avatar_revision: Option<String>,
    pub banner_revision: Option<String>,
    pub bio: Option<String>,
    pub connections: Option<Connections>,
    pub friend_count: Option<usize>,
    pub distinguishment: Option<Distinguishment>,
}

impl TetrUser {
    pub fn id(&self) -> String {
        self.id.clone().unwrap()
    }

    pub fn username(&self) -> String {
        self.username
            .clone()
            .unwrap_or(walao!(unwrap, parse))
    }

    pub fn role(&self) -> String {
        self.role
            .clone()
            .unwrap_or(walao!(unwrap, parse))
    }

    pub fn timestamp(&self) -> String {
        self.ts
            .clone()
            .unwrap_or(walao!(unwrap, parse))
    }

    pub fn bot_info(&self) -> Option<String> {
        if let Some(s) = self.botmaster.clone() {
            return Some(s);
        }
        None
    }

    pub fn get_badges(&self) -> Option<Vec<Badge>> {
        if self.badges.is_empty() {
            return None;
        }
        let mut v = Vec::new();
        for badge in &self.badges {
            v.push(badge.clone().unwrap());
        }
        Some(v)
    }

    pub fn stats(&self) -> (f64, isize, isize, f64) {
        (self.xp.unwrap(), self.gamesplayed.unwrap(), self.gameswon.unwrap(), self.gametime.unwrap())
    }

    pub fn country(&self) -> (String, String) {
        let s = self.country
            .clone()
            .unwrap_or(String::from("null"));
        let t = if s.as_str() == "null" { String::from("null") } else { format!(":flag_{}:", s.to_ascii_lowercase()) };
        (s, t)
    }

    pub fn league(&self) -> League {
        let w: String = ExpectError::ParseError.into();
        self.league.clone().expect(walao!(expect, w).as_ref())
    }
}

#[derive(Deserialize, Clone, Debug)]
pub struct Badge {
    pub id: Option<String>,
    pub label: Option<String>,
    pub ts: Option<String>,
}

impl fmt::Display for Badge {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ID: {} Label: {} Timestamp (UTC): {}\n", self.clone().id.unwrap_or(walao!(unwrap, parse)), self.clone().label.unwrap_or(walao!(unwrap, parse)), self.clone().ts.unwrap_or(walao!(unwrap, parse)))
    }
}

#[derive(Deserialize, Clone, Debug)]
pub struct League {
    pub gamesplayed: Option<usize>,
    pub gameswon: Option<usize>,
    pub rating: Option<f64>,
    pub rank: Option<String>,
    pub bestrank: Option<String>,
    pub standing: Option<isize>,
    pub standing_local: Option<isize>,
    pub next_rank: Option<String>,
    pub prev_rank: Option<String>,
    pub next_at: Option<isize>,
    pub prev_at: Option<isize>,
    pub percentile: Option<f64>,
    pub percentile_rank: Option<String>,
    pub glicko: Option<f64>,
    pub rd: Option<f64>,
    pub apm: Option<f64>,
    pub pps: Option<f64>,
    pub vs: Option<f64>,
    pub decaying: Option<bool>,
}

impl League {
    pub fn games(&self) -> (usize, usize) {
        (self.gameswon.unwrap(), self.gamesplayed.unwrap())
    }
    pub fn rank_info(&self) -> (String, String, String, String) {
        (
            self.rank.clone().unwrap(),
            self.bestrank.clone().unwrap(),
            self.next_rank.clone().unwrap(),
            self.prev_rank.clone().unwrap()
        )
    }
    pub fn percentile(&self) -> f64 {
        self.percentile.unwrap_or(10.0) * 100.0
    }
    pub fn glicko(&self) -> (f64, f64) {
        (self.glicko.unwrap(), self.rd.unwrap())
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct Connections {
    pub discord: Option<DiscordConnection>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct DiscordConnection {
    pub id: Option<String>,
    pub username: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Distinguishment {
    pub _type: Option<String>,
}