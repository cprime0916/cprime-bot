use serde::Deserialize;

#[derive(Deserialize)]
pub struct ContestInfo{
    pub meta: Option<Meta>,
    pub objects: Option<Vec<Contest>>,
}

#[derive(Deserialize)]
pub struct Meta{
    pub limit: Option<usize>,
    pub next: Option<String>,
    pub offset: Option<usize>,
    pub previous: Option<String>,
    pub total_count: Option<usize>,
}

#[derive(Deserialize)]
pub struct Contest{
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