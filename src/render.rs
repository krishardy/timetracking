use super::analyze::Statistics;
use std::io::Result;

pub fn render(statistics: &Statistics) -> Result<()> {
    // Render statistics
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
