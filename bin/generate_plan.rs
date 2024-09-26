use std::fs::File;
use std::io::{self, BufWriter};

use weekly_planner::weekplan::{Time, Weekday};
use weekly_planner::WeekPlan;

fn main() -> Result<(), io::Error> {
    let start = Time::new(8, 30).unwrap();
    let week_plan = {
        let mut w = WeekPlan::new(start, 90, 7).unwrap();
        w.try_insert_range(
            Weekday::Monday,
            (Time::new(10, 0).unwrap(), 2),
            "Computer architectures".to_owned(),
        )
        .unwrap()
        .try_insert_range(
            Weekday::Tuesday,
            (Time::new(8, 30).unwrap(), 2),
            "Computer architectures".to_owned(),
        )
        .unwrap();
        w
    };

    println!("Writing to data/plan.json");

    let outfile = File::create("data/plan.json")?;
    let writer = BufWriter::new(outfile);
    serde_json::to_writer_pretty(writer, &week_plan)?;

    Ok(())
}
