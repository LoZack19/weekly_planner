mod weekplan;

use weekplan::{Time, WeekPlan, Weekday};

fn main() {
    let mut weekplan = WeekPlan::new(Time::new(8, 30).unwrap(), 90, 7).unwrap();

    weekplan
        .try_insert(
            Weekday::Monday,
            Time::new(8, 30).unwrap(),
            "activity".into(),
        )
        .unwrap();

    println!("{}", weekplan.to_html());
}
