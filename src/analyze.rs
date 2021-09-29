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

use chrono::prelude::*;
use std::collections::BTreeMap;
use log::{debug, error};
use std::cmp::max;
use std::collections::btree_map::Entry;

use super::model::TimesheetRecord;

type ProjectTimeMap = BTreeMap<String, chrono::Duration>;

type DateProjectsMap = BTreeMap<chrono::Date<chrono::Local>, ProjectTimeMap>;

#[derive(Debug)]
pub struct Statistics {
    pub date_projects: DateProjectsMap,
    pub max_project_len: usize
}

impl Statistics {
    pub fn new() -> Self {
        Self {
            date_projects: DateProjectsMap::new(),
            max_project_len: 0
        }
    }

    pub fn calculate(&mut self, infile: &str) -> Result<(), Box<dyn std::error::Error>> {
        let timestamp_format = "%Y-%m-%d %H:%M";
        let time_format = "%H:%M";
    
        let mut rdr = csv::Reader::from_path(infile)?;
        for result in rdr.deserialize() {
            let record: TimesheetRecord = result?;
            match record.submitted.as_str() {
                "y" | "yes" | "true" => continue,
                "n" | "no" | "false" => continue,
                _ => {
                    debug!("{:?}", record);
    
                    let start: chrono::DateTime<chrono::Local>;
                    self.update_max_project_len(record.project.len());
                    
                    // start must be a datetime
                    match Local.datetime_from_str(record.start.as_str(), timestamp_format) {
                        Ok(d) => start = d,
                        Err(e) => {
                            error!("start time [{}] cannot be parsed with format [{}]: {}", record.start, timestamp_format, e);
                            continue;
                        }
                    }
    
                    // end can be a date or a datetime
                    let end: chrono::DateTime<chrono::Local>;
                    match Local.datetime_from_str(record.end.as_str(), timestamp_format) {
                        Ok(d) => end = d,
                        Err(_) => {
                            // Is this just a time instead?
                            match NaiveTime::parse_from_str(record.end.as_str(), time_format) {
                                Ok(d) => {
                                    end = Local.ymd(start.year(), start.month(), start.day()).and_hms(d.hour(), d.minute(), d.second());
                                },
                                Err(e) => {
                                    error!("end time [{}] cannot be parsed with format [{}] or [{}]: {}", record.end, timestamp_format, time_format, e);
                                    continue;        
                                }
                            }
                        }
                    }
    
                    let date: chrono::Date<chrono::Local> = start.date();
                    match self.date_projects.entry(date) {
                        Entry::Occupied(project_time_map) => {
                            match project_time_map.into_mut().entry(record.project) {
                                Entry::Occupied(o) => {
                                    let entry = o.into_mut();
                                    *entry = *entry + (end - start);
                                },
                                Entry::Vacant(v) => {
                                    v.insert(end - start);
                                }
                            }
                        },
                        Entry::Vacant(v) => {
                            let mut map = ProjectTimeMap::new();
                            map.insert(record.project, end - start);
                            v.insert(map);
                        }
                    }
                },
            }
        }
        Ok(())
    }

    fn update_max_project_len(&mut self, len: usize) {
        self.max_project_len = max(self.max_project_len, len);
    }
}


