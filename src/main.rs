use std::fs::File;
use std::io::{self, BufReader, Write};

use weekly_planner::WeekPlan;

fn main() -> Result<(), io::Error> {
    let week_plan: WeekPlan = {
        let infile = File::open("data/plan.json")?;
        let reader = BufReader::new(infile);
        serde_json::from_reader(reader)?
    };

    let mut outfile = File::create("output/week_plan.html")?;
    write!(outfile, "{}", week_plan.to_html())?;
    Ok(())
}
