// src/logger.rs

use log;
use std::env;
use std::fs;

pub fn log_writer_init() -> Result<String, fern::InitError> {
    let log_level = env::var("LOG_LEVEL").unwrap_or("INFO".into());
    let log_level = log_level
        .parse::<log::LevelFilter>()
        .unwrap_or(log::LevelFilter::Info);

    let mut builder = fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "[{}] [{}] [{}] {}",
                chrono::Local::now().format("%Y-%m-%d %H:%M"),
                record.target(),
                record.level(),
                message
            ))
        })
        .level(log_level)
        // log to stderr
        .chain(std::io::stderr());
    let timest = chrono::Local::now().format("%Y-%m-%d %H:%M").to_string();
    let file_name = format!("{}.log", timest);
    let log_file = fs::File::create(&file_name)?;
    builder = builder.chain(log_file);
    // globally apply logger
    builder.apply()?;
    // trace!("TRACE output enabled");
    // debug!("DEBUG output enabled");
    // info!("INFO output enabled");
    // warn!("WARN output enabled");
    // error!("ERROR output enabled");
    Ok(file_name)
}
