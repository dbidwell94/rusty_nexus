mod mods;

use std::rc::Rc;

use mods::Mods;
use raxios::{map_string, Raxios, RaxiosConfig};

pub type NexusApiResult<T> = anyhow::Result<T>;

const NEXUS_API_BASE_URL: &'static str = "https://api.nexusmods.com/";

pub struct NexusApi {
    pub mods: Mods,
}

impl NexusApi {
    pub fn new(api_key: &str) -> Self {
        let default_headers = map_string! {apikey : api_key};

        let raxios = Raxios::new(
            NEXUS_API_BASE_URL,
            Some(RaxiosConfig {
                headers: Some(default_headers),
                ..Default::default()
            }),
        )
        .unwrap();

        let raxios = Rc::new(raxios);

        let mods = Mods::from(&raxios);

        Self { mods }
    }
}
