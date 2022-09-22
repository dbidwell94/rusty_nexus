use std::fmt::Display;

use serde::{Serialize, Deserialize};

#[derive(Clone)]
pub enum ModFileCategory {
    Main,
    Update,
    Options,
    OldVersion,
    Misc,
}

impl Display for ModFileCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ModFileCategory::Main => write!(f, "main"),
            ModFileCategory::Update => write!(f, "update"),
            ModFileCategory::Options => write!(f, "optional"),
            ModFileCategory::OldVersion => write!(f, "old_version"),
            ModFileCategory::Misc => write!(f, "miscellaneous"),
        }
    }
}

impl From<&ModFileCategory> for ModFileCategory {
    fn from(mfc: &ModFileCategory) -> Self {
        mfc.clone()
    }
}

impl From<ModFileCategory> for String {
    fn from(mfc: ModFileCategory) -> Self {
        format!("{mfc}")
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ListFilesResponse {
    files: Vec<GameFileInfo>,
    file_updates: Vec<GameFileUpdatesInfo>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GameFileInfo {
    id: Vec<u32>,
    uid: u64,
    file_id: u32,
    name: String,
    version: String,
    category_id: u16,
    category_name: Option<String>,
    is_primary: bool,
    size: u32,
    file_name: String,
    uploaded_timestamp: u64,
    uploaded_time: String,
    mod_version:String,
    external_virus_scan_url: String,
    description: String,
    size_kb: u16,
    size_in_bytes: u64,
    changelog_html: Option<String>,
    content_preview_link: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GameFileUpdatesInfo {
    old_file_id: u32,
    new_file_id: u32,
    old_file_name: String,
    new_file_name: String,
    uploaded_timestamp: u64,
    uploaded_time: String
}