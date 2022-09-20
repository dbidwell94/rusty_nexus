use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct UpdatedModInfo {
    mod_id: u32,
    latest_file_update: usize,
    latest_mod_activity: usize,
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
