extern crate clap;

use clap::{App, Arg};
use log::{debug, error, info, trace, warn};

mod model;
mod analyze;
mod render;

use crate::analyze::Statistics;
use crate::render::render;

const APPNAME: &str = "timetracking";
const VERSION: &str = "0.0.1";

fn main() {
    let matches = App::new(APPNAME)
        .version(VERSION)
        .author("Kris Hardy <hardyrk@gmail.com>")
        .about("Generates reports from timetracking CSV files")
        .arg(
            Arg::with_name("verbose")
                .short("v")
                .multiple(true)
                .help("Increase logging verbosity (can be used multiple times)")
        )
        .args_from_usage("<infile> 'Input CSV file'")
        .get_matches();

    let level = match matches.occurrences_of("verbose") {
        0 => log::LevelFilter::Error,
        1 => log::LevelFilter::Warn,
        2 => log::LevelFilter::Info,
        3 | _ => log::LevelFilter::Debug,
    };
    
    setup_logger(level).unwrap();
    warn!("Logging level set to: {}", level);

    if let Some(infile) = matches.value_of("infile") {
        let mut stats = Statistics::new();
        match stats.calculate(infile) {
            Ok(_) => render(&stats).unwrap(),
            Err(err) => error!("An error was returned during processing of the input file. {}", err),
        }
    } else {
        error!("infile parameter was not provided");
    }
}

fn setup_logger(level: log::LevelFilter) -> Result<(), fern::InitError> {
    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "{}|{}|{}|{}",
                chrono::Local::now().format("%Y-%m-%dT%H:%M:%S"),
                record.target(),
                record.level(),
                message
            ))
        })
        .level(level)
        .chain(std::io::stdout())
        .apply()?;
    Ok(())
}