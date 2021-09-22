use chrono::prelude::*;
use std::collections::BTreeMap;
use log::{debug, error, info, trace, warn};
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
    
        let mut rdr = csv::Reader::from_path(infile)?;
        for result in rdr.deserialize() {
            let record: TimesheetRecord = result?;
            self.update_max_project_len(record.project.len());
            match record.submitted.as_str() {
                "y" | "yes" | "true" => continue,
                _ => {
                    debug!("{:?}", record);
    
                    let mut start: chrono::DateTime<chrono::Local> = chrono::Local::now();
                    match Local.datetime_from_str(record.start.as_str(), timestamp_format) {
                        Ok(d) => start = d,
                        Err(e) => error!("start time [{}] cannot be parsed with format [{}]: {}", record.start, timestamp_format, e)
                    }
    
                    let mut end: chrono::DateTime<chrono::Local> = chrono::Local::now();
                    match Local.datetime_from_str(record.end.as_str(), timestamp_format) {
                        Ok(d) => end = d,
                        Err(e) => error!("end time [{}] cannot be parsed with format [{}]: {}", record.end, timestamp_format, e)
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


