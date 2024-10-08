#[macro_use]
extern crate lazy_static;

pub mod db;

pub mod error;
pub mod eval_events;
pub mod github;
pub mod types;

use types::{FileConfig, GlobalConfig};

pub fn read_config() -> GlobalConfig {
    // TODO: actually read a config
    let file_config = FileConfig { cache_dir: None };

    GlobalConfig::new(&file_config)
}
