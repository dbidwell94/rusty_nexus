use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct UpdatedModInfo {
    pub mod_id: u32,
    pub latest_file_update: usize,
    pub latest_mod_activity: usize,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ModInfoResponse {
    pub name: Option<String>,
    pub summary: Option<String>,
    pub description: Option<String>,
    pub picture_url: Option<String>,
    pub mod_downloads: Option<u32>,
    pub mod_unique_downloads: Option<u32>,
    pub uid: u64,
    pub mod_id: u32,
    pub game_id: u32,
    pub allow_rating: bool,
    pub domain_name: String,
    pub category_id: u16,
    pub version: String,
    pub endorsement_count: u32,
    pub created_timestamp: u64,
    pub created_time: String,
    pub updated_timestamp: u64,
    pub updated_time: String,
    pub author: String,
    pub uploaded_by: String,
    pub uploaded_users_profile_url: Option<String>,
    pub contains_adult_content: bool,
    pub status: String,
    pub available: bool,
    pub user: Option<UserInfo>,
    pub endorsement: Option<EndorsementInfo>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserInfo {
    pub member_id: u32,
    pub member_group_id: u16,
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EndorsementInfo {
    pub endorse_status: String,
    pub timestamp: Option<u64>,
    pub version: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ModEndorsementResult {
    pub message: String,
    pub status: String,
}

#[derive(Clone)]
pub enum Period {
    Day,
    Week,
    Month,
}

impl Display for Period {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Period::Day => write!(f, "1d"),
            Period::Week => write!(f, "1w"),
            Period::Month => write!(f, "1m"),
        }
    }
}

impl From<Period> for String {
    fn from(p: Period) -> Self {
        format!("{p}")
    }
}
