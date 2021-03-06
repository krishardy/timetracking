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
use std::collections::btree_map::Entry;
use csv::{ReaderBuilder};

use super::model::{TimesheetRecord, ParsedTimesheetRecord};

type ProjectTimeMap = BTreeMap<String, chrono::Duration>;

type DateProjectsMap = BTreeMap<chrono::Date<chrono::Local>, ProjectTimeMap>;

#[derive(Debug)]
pub struct Statistics {
    pub date_projects: DateProjectsMap,
}

impl Statistics {
    pub fn new() -> Self {
        Self {
            date_projects: DateProjectsMap::new(),
        }
    }

    pub fn calculate(&mut self, infile: &str, ignore_submitted: bool) -> Result<(), Box<dyn std::error::Error>> {
        let mut prev_record = ParsedTimesheetRecord {
            submitted: String::new(),
            project: String::new(),
            start: Local::now(),
            end: Some(Local::now()),
            notes: String::new(),
            deferred: false,
        };

        let mut rdr = ReaderBuilder::new()
            .delimiter(b',')
            .comment(Some(b'#'))
            .from_path(infile)
            .expect("infile could not be read");

        for result in rdr.deserialize() {
            match result {
                Ok(r) => {
                    let model: TimesheetRecord = r;
                    self.parse_record(model, &mut prev_record, ignore_submitted).unwrap_or_default();
                },
                Err(ref e) => {
                    error!("Row could not be parsed: {:?}", e);                  
                },
            }
        }

        // If previous record does not have an end date/time, show a warning
        if prev_record.deferred {
            error!("The last record did not have an end time set, so is using a default value of Local:now()");
            prev_record.end = Some(Local::now());
            self.accumulate_record(&prev_record)
        }

        Ok(())
    }

    fn parse_record(&mut self, model: TimesheetRecord, prev_record: &mut ParsedTimesheetRecord, ignore_submitted: bool) -> Result<(), Box<dyn std::error::Error>> {
        let timestamp_format = "%Y-%m-%d %H:%M";
        let time_format = "%H:%M";
    
        let mut parsed_record = ParsedTimesheetRecord {
            submitted: model.submitted.clone(),
            project: model.project.clone(),
            start: Local::now(),
            end: None,
            notes: model.notes.clone(),
            deferred: false,
        };

        debug!("{:?}", model);

        // start must be a datetime or a time
        // If start is a time, use the previously parsed date
        parsed_record.start = match Local.datetime_from_str(model.start.as_str(), timestamp_format) {
            Ok(dt) => dt,
            Err(_) => {
                match NaiveTime::parse_from_str(model.start.as_str(), time_format) {
                    Ok(t) => {
                        Local.ymd(prev_record.start.year(), prev_record.start.month(), prev_record.start.day())
                            .and_hms(t.hour(), t.minute(), t.second())
                    },
                    Err(e) => {
                        error!("start time [{}] cannot be parsed with format [{}] or [{}]: {}", model.start, timestamp_format, time_format, e);
                        return Ok(());
                        //continue;
                    }
                }
            }
        };
        
        // If end is empty, defer calculating the end until the next record is read and use its start time
        //let mut end : Option<DateTime<Local>>;
        parsed_record.end = match model.end {
            Some(e) => {
                match self.parse_end_date(e.as_str(), parsed_record.start, timestamp_format, time_format) {
                    Ok(end_dt) => Some(end_dt),
                    Err(e) => {
                        error!("end time [{}] cannot be parsed with format [{}] or [{}]: {}", e, timestamp_format, time_format, e);
                        return Ok(());
                        //continue;        
                    },
                }
            },
            None => {
                parsed_record.deferred = true;
                None
            },
        };

        match prev_record.deferred {
            true => {
                // Set the end time to the start time of the current record and process the prev_record
                prev_record.end = Some(parsed_record.start.clone());
                
                let mut accumulate = true;
                if !ignore_submitted {
                    match prev_record.submitted.as_str() {
                        "y" | "yes" | "true" => {
                            accumulate = false;
                        }
                        "n" | "no" | "false" => {
                            accumulate = false;
                        }
                        _ => {}
                    }
                }
                if accumulate {
                    self.accumulate_record(&prev_record)
                }
            },
            false => {
                // Nothing to do
            }
        }

        if !parsed_record.deferred {
            let mut accumulate = true;
            if !ignore_submitted {
                match parsed_record.submitted.as_str() {
                    "y" | "yes" | "true" => {
                        accumulate = false;
                    }
                    "n" | "no" | "false" => {
                        accumulate = false;
                    }
                    _ => {}
                }
            }
            if accumulate {
                self.accumulate_record(&parsed_record)
            }
        }

        prev_record.clone_from(&parsed_record);
        Ok(())
    }

    fn parse_end_date(&mut self, end: &str, start: DateTime<Local>, timestamp_format: &str, time_format: &str) -> Result<DateTime<Local>, chrono::ParseError> {
        // end can be a time or a datetime
        let end_datetime = match Local.datetime_from_str(end, timestamp_format) {
            Ok(d) => d,
            Err(_) => {
                // Is this just a time instead?
                match NaiveTime::parse_from_str(end, time_format) {
                    Ok(d) => Local.ymd(start.year(), start.month(), start.day()).and_hms(d.hour(), d.minute(), d.second()),
                    Err(e) => {
                        return Err(e)
                    }
                }
            }
        };
        Ok(end_datetime)
    }

    fn accumulate_record(&mut self, record: &ParsedTimesheetRecord) {
        match self.date_projects.entry(record.start.date()) {
            Entry::Occupied(project_time_map) => {
                // The date is already in date_projects. Add to it.
                match project_time_map.into_mut().entry(record.project.clone()) {
                    Entry::Occupied(o) => {
                        // the project is already in project_time_map. Add to it.
                        let entry = o.into_mut();
                        let time_delta = record.end.unwrap() - record.start;
                        if time_delta < chrono::Duration::zero() {
                            error!("Skipping record due to a negative time delta of {} minutes: Project={}, Start={}, End={}", time_delta.num_minutes(), record.project, record.start, record.end.unwrap());
                        } else {
                            *entry = *entry + (record.end.unwrap() - record.start);
                        }
                    },
                    Entry::Vacant(v) => {
                        // the project is not yet in project_time_map. Initialize it.
                        v.insert(record.end.unwrap() - record.start);
                    }
                }
            },
            Entry::Vacant(v) => {
                // The date is not yet in the date_projects. Initialize it.
                let mut map = ProjectTimeMap::new();
                map.insert(record.project.clone(), record.end.unwrap() - record.start);
                v.insert(map);
            }
        }
    }
}


