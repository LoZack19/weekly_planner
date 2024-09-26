mod weekplan;

use std::fs::File;
use std::io::{self, BufReader};
use weekplan::WeekPlan;

fn main() -> Result<(), io::Error> {
    let week_plan: WeekPlan = {
        let infile = File::open("data/plan.json")?;
        let reader = BufReader::new(infile);
        serde_json::from_reader(reader)?
    };

    println!("{}", week_plan.to_html());
    Ok(())
}
