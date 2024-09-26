use std::fs::File;
use std::io::{self, BufWriter};
use std::str::FromStr;

use weekly_planner::weekplan::{Time, Weekday};
use weekly_planner::{poli_plan, WeekPlan};

fn main() -> Result<(), io::Error> {
    let start = Time::new(8, 30).unwrap();
    let week_plan = poli_plan! {
        start, 90, 7,
        "Monday" => "10:00", 2, "Computer architectures",
        "Tuesday" => "8:30", 2, "Computer architectures",
        "Tuesday" => "11:30", 2, "Electronics",
        "Wednesday" => "8:30", 2, "Electronics",
        "Thursday" => "11:30", 1, "Electronics",
        "Monday" => "16:00", 1, "Operating systems",
        // "Thursday" => "11:30", 1, "Operating systems",
        "Friday" => "8:30", 2, "Operating systems",
        "Thursday" => "13:00", 1, "Simulation",
        "Friday" => "14:30", 1, "Simulation",
    };

    println!("Writing to data/plan.json");

    let outfile = File::create("data/plan.json")?;
    let writer = BufWriter::new(outfile);
    serde_json::to_writer_pretty(writer, &week_plan)?;

    Ok(())
}
