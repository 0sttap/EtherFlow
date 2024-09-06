// src/logger.rs

use std::env;
//use fern::colors::{Color, ColoredLevelConfig};

pub fn init() -> Result<(), fern::InitError> {
    // pull log level from env
    let log_level = env::var("LOG_LEVEL").unwrap_or("INFO".into());
    let log_level = log_level
        .parse::<log::LevelFilter>()
        .unwrap_or(log::LevelFilter::Info);


    let mut builder = fern::Dispatch::new()
        .format(|out, message, record| {

            // Create a colored level configuration
            // let colors = ColoredLevelConfig::new()
            //     .error(Color::Red)
            //     .warn(Color::Yellow)
            //     .info(Color::Green)
            //     .debug(Color::Cyan)
            //     .trace(Color::White);

            let input = format!(
                "[{}][{}][{}] {}", 
                chrono::Local::now().format("%m-%d-%Y %H:%M:%S"),
                record.target(),
                record.level(), // colors.color(record.level()),
                message.to_string()
            );

            let output = input.replace("\n", &format!(
                    "\n[{}][{}][{}] ", 
                    chrono::Local::now().format("%m-%d-%Y %H:%M:%S"), 
                    record.target(),
                    record.level(), // colors.color(record.level())
                )
            );
        
            out.finish(format_args!("{}", output))
        })
        .level(log_level)
        .filter(|metadata| {
            let target = metadata.target();
            target == "ts::api" || target == "ts::evm" || target == "ts::simulate"
        })
        .chain(std::io::stderr());

    // also log to file if one is provided via env
    if let Ok(log_file) = env::var("LOG_FILE") {
        builder = builder.chain(fern::log_file(log_file)?);
    }

    // globally apply logger
    builder.apply()?;

    Ok(())
}