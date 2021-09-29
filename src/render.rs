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

use super::analyze::Statistics;
use std::io::Result;

pub fn render(statistics: &Statistics) -> Result<()> {
    // Render statistics
    println!("=== REPORT ===");
    for (date, project_time_map) in statistics.date_projects.iter() {
        println!("{}", date.format("%Y-%m-%d"));
        let mut sum = 0;
        for (project, duration) in project_time_map.iter() {
            let total_mins = duration.num_minutes();
            sum = sum + total_mins;
            let hours = total_mins / 60;
            let mins = total_mins % 60;
            println!("  {:width$} | {:02}:{:02}", project, hours, mins, width=statistics.max_project_len);
        }
        let hours = sum / 60;
        let mins = sum % 60;
        println!("  {:->width$} | {:02}:{:02}", "SUM", hours, mins, width=statistics.max_project_len);
        println!("{:-<width$}", "", width=statistics.max_project_len+10);
    }
    Ok(())
}
