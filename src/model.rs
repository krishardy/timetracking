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

use serde::{Serialize, Deserialize};
use chrono::{DateTime, Local};

#[derive(Debug, Serialize, Deserialize)]
pub struct TimesheetRecord {
    pub submitted: String,
    pub project: String,
    pub start: String, // chrono::DateTime<chrono::Local>
    pub end: Option<String>, //Optional chrono::DateTime<chrono::Local>
    pub notes: String,
}

#[derive(Debug, Clone)]
pub struct ParsedTimesheetRecord {
    pub project: String,
    pub start: DateTime<Local>,
    pub end: Option<DateTime<Local>>,
    pub notes: String,
    pub deferred: bool,
}
