/*
Timetracking generates reports from timesheet files
Copyright (C) 2021  Kris Hardy

Timetracking is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

Timetracking is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with Timetracking.  If not, see <https://www.gnu.org/licenses/>.
*/

use clap::{App, Arg};
use log::{error, warn};
use git_version::git_version;

mod model;
mod analyze;
mod render;

use crate::analyze::Statistics;
use crate::render::render;

const APPNAME: &str = "timetracking";
const VERSION: &str = "0.1.0";
const GITVERSION: &str = git_version!();

fn main() {
    let matches = App::new(APPNAME)
        .version(format!("{} ({})", VERSION, GITVERSION).as_str())
        .author("Copyright (C) 2021  Kris Hardy <hardyrk@gmail.com>")
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