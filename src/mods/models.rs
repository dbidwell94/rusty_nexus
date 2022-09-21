use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct UpdatedModInfo {
    mod_id: u32,
    latest_file_update: usize,
    latest_mod_activity: usize,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ModInfoResponse {
    name: Option<String>,
    summary: Option<String>,
    description: Option<String>,
    picture_url: Option<String>,
    mod_downloads: Option<u32>,
    mod_unique_downloads: Option<u32>,
    uid: u64,
    mod_id: u32,
    game_id: u32,
    allow_rating: bool,
    domain_name: String,
    category_id: u16,
    version: String,
    endorsement_count: u32,
    created_timestamp: u64,
    created_time: String,
    updated_timestamp: u64,
    updated_time: String,
    author: String,
    uploaded_by: String,
    uploaded_users_profile_url: Option<String>,
    contains_adult_content: bool,
    status: String,
    available: bool,
    user: Option<UserInfo>,
    endorsement: Option<EndorsementInfo>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserInfo {
    member_id: u32,
    member_group_id: u16,
    name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EndorsementInfo {
    endorse_status: String,
    timestamp: Option<u64>,
    version: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ModEndorsementResult {
    message: String,
    status: String,
}

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
