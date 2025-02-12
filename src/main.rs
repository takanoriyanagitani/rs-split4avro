use std::process::ExitCode;

use std::io;

use rs_split4avro::Config;
use rs_split4avro::SPLIT_COUNT_DEFAULT;

fn env_val_by_key(key: &'static str) -> Result<String, io::Error> {
    std::env::var(key).map_err(|e| io::Error::other(format!("env var {key} unknown: {e}")))
}

fn dir_name() -> Result<String, io::Error> {
    env_val_by_key("ENV_OUTPUT_DIR_NAME")
}

fn split_count() -> u32 {
    env_val_by_key("ENV_SPLIT_COUNT")
        .ok()
        .and_then(|s| str::parse(s.as_str()).ok())
        .unwrap_or(SPLIT_COUNT_DEFAULT)
}

fn config() -> Result<Config, io::Error> {
    dir_name().map(|dirname| Config {
        dirname,
        split_count: split_count(),
    })
}

fn sub() -> Result<(), io::Error> {
    config()?.stdin2values2fs_default()
}

fn main() -> ExitCode {
    sub().map(|_| ExitCode::SUCCESS).unwrap_or_else(|e| {
        eprintln!("{e}");
        ExitCode::FAILURE
    })
}
