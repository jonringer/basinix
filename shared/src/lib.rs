#[macro_use]
extern crate lazy_static;

pub mod db;

pub mod error;
pub mod eval_events;
pub mod github;
pub mod types;

use types::{FileConfig, GlobalConfig};

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

pub fn read_config() -> GlobalConfig {
    // TODO: actually read a config
    let file_config = FileConfig { cache_dir: None };

    GlobalConfig::new(&file_config)
}
