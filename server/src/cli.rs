use clap::{App, AppSettings, Arg, ArgMatches, SubCommand};
use toml;
use log::{error, info};

use basinix_shared::types::{FileConfig, GlobalConfig};

pub fn build_cli() -> App<'static, 'static> {
    App::new("basinix-server")
        .version("0.1.0")
        .author("Jon Ringer <jonringer117@gmail.com>")
        .about("Nixpkgs pr review tool")
        .setting(AppSettings::ColoredHelp)
        // make it so that completions subcommand doesn't
        // inherit global options
        .setting(AppSettings::ArgsNegateSubcommands)
        .after_help(
            "ENV VARS:

    GITHUB_TOKEN\tToken used during github api calls.
    XDG_CACHE_DIR\tDefault directory for nixpkgs and PR checkouts.

EXAMPLES:

# basinix server with configuration file
$ basinix-server --config /etc/basinix/config.toml

# explicit argument passing to basinix-server
$ basinix-server --workers 5 --cores 2 --cache-dir=/var/lib/basinix/
",
        )
        .arg(Arg::from_usage(
            "-c,--config [file_path] 'File path to configuration file, use --generate-config for default example`'",
            ))
        .arg(Arg::from_usage(
            "--generate-config [file_path] 'Generate a configuration file with default values'",
            ).default_value("CHANGE"))
        .arg(Arg::from_usage(
            "-w,--workers [num] 'Number of workers to spawn as child threads, each worker will build a single derivation at-a-time'",
            ))
        .arg(Arg::from_usage(
            "-j,--cores [cores] 'Number of cores for each worker. Similar to --cores nix option'",
            ))
        .arg(Arg::from_usage(
            "--cache-dir [directory] 'Directory used for nixpkgs and pr worktrees'",
            ))
        .subcommand(
            SubCommand::with_name("completions")
                .about("Generate shell completion scripts, writes to stdout")
                .arg(
                    Arg::from_usage("<SHELL>")
                        .case_insensitive(true)
                        .possible_values(&clap::Shell::variants()),
                ),
        )
}

pub fn read_args(
  matches: &ArgMatches
  ) -> GlobalConfig {
    let file_config: FileConfig = match matches.value_of("config_file") {
        Some(file_path) => {
            match toml::from_str(&std::fs::read_to_string(file_path).unwrap()) {
                Ok(config) => {
                    // good config
                    config
                },
                Err(e) => {
                    error!("Unable to read config, at {}", e);
                    std::process::exit(1);
                }
            }
        },
        None => {
            info!("No configuration file passed, checking other arguments");

            if let Some(cache_dir) = matches.value_of("cache-dir") {
                FileConfig { cache_dir: Some(cache_dir.to_string()) }
            } else {
                info!("No configuration file or cache dir was passed, defaulting to XDG_CACHE_DIR");
                FileConfig { cache_dir: None }
            }
        }
    };

    GlobalConfig::new(&file_config)
    // TODO: override with explicitly passed options
}
