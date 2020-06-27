
use anyhow::{ Result };

use fern;
use log;

pub fn start() -> Result<()> {
    let config = fern::Dispatch::new();
    let time = chrono::Local::now().format("%Y-%m-%d_%H:%M:%S");
    config.level(log::LevelFilter::Trace)
        .format(|out, message, record| {
            out.finish(format_args!(
                "{}:[{}][{}]:- {}",
                chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
                record.target(),
                record.level(),
                message
            ))
        })
        .chain(fern::log_file(format!("log-{}", time))?)
        .apply()?;
    Ok(())
}

