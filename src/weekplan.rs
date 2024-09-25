use std::collections::HashMap;
use std::fmt::Write;

pub use activity::Activity;
pub use time::Time;
pub use weekday::Weekday;

mod activity;
mod time;
mod weekday;

#[derive(Debug)]
pub struct WeekPlan {
    plan: HashMap<(Weekday, Time), Activity>,
    start: Time,
    slot_duration: u16,
    slots: u8,
}

#[derive(Debug)]
pub enum Error {
    InvalidSlot,
    AlreadyBooked,
}

type Result<T> = std::result::Result<T, Error>;

impl WeekPlan {
    pub fn new(start: Time, slot_duration: u16, slots: u8) -> Option<Self> {
        start.try_sum(u16::from(slots) * slot_duration)?;

        Some(WeekPlan {
            plan: HashMap::new(),
            start,
            slot_duration,
            slots,
        })
    }

    pub fn is_valid_slot(&self, slot: Time) -> bool {
        let slot = slot.to_minutes();
        let start = self.start.to_minutes();

        (slot - start) % u16::from(self.slot_duration) == 0
            && u8::try_from((slot - start) / u16::from(self.slot_duration)).unwrap() < self.slots
    }

    pub fn insert(&mut self, weekday: Weekday, slot: Time, activity: Activity) -> Result<()> {
        if !self.is_valid_slot(slot) {
            return Err(Error::InvalidSlot);
        }

        let key = (weekday, slot);

        if self.plan.contains_key(&key) {
            return Err(Error::AlreadyBooked);
        }

        self.plan.insert(key, activity);

        Ok(())
    }

    pub fn insert_range(
        &mut self,
        weekday: Weekday,
        slots: (Time, u8),
        activity: Activity,
    ) -> Result<()> {
        let (start, len) = slots;

        for i in 0..len {
            self.insert(
                weekday,
                start
                    .try_sum(u16::from(i) * self.slot_duration)
                    .ok_or(Error::InvalidSlot)?,
                activity.clone(),
            )?;
        }

        Ok(())
    }

    fn to_table(&self) -> (Vec<Weekday>, Vec<Time>, Vec<Activity>) {
        let default_activity: Activity = "".into();

        let weekdays = vec![
            Weekday::Monday,
            Weekday::Tuesday,
            Weekday::Wednesday,
            Weekday::Thursday,
            Weekday::Friday,
            Weekday::Saturday,
            Weekday::Sunday,
        ];

        let times: Vec<Time> = (0..self.slots)
            .map(|num| {
                self.start
                    .try_sum(u16::from(num) * self.slot_duration)
                    .unwrap()
            })
            .collect();

        let table: Vec<Activity> = weekdays
            .iter()
            .map(|&weekday| {
                times
                    .iter()
                    .map(|&time| self.plan.get(&(weekday, time)).unwrap_or(&default_activity))
                    .map(|s| s.to_owned())
                    .collect::<Vec<Activity>>()
            })
            .reduce(|mut acc, v| {
                v.into_iter().for_each(|value| acc.push(value));
                acc
            })
            .unwrap();

        (weekdays, times, table)
    }

    pub fn to_html(&self) -> String {
        let mut html = String::new();
        let (weekdays, times, table) = self.to_table();

        writeln!(html, "<table border='1'>").unwrap();

        // Write header row
        writeln!(html, "  <tr>").unwrap();
        writeln!(html, "    <th></th>").unwrap();
        for day in &weekdays {
            writeln!(html, "    <th>{:?}</th>", day).unwrap();
        }
        writeln!(html, "  </tr>").unwrap();

        // Write table rows
        for (i, time) in times.iter().enumerate() {
            writeln!(html, "  <tr>").unwrap();
            writeln!(html, "    <th>{:02}:{:02}</th>", time.hour(), time.minute()).unwrap();

            for j in 0..weekdays.len() {
                writeln!(html, "    <td>").unwrap();
                writeln!(html, "      {}", table[i * times.len() + j]).unwrap();
                writeln!(html, "    </td>").unwrap();
            }

            writeln!(html, "  </tr>").unwrap();
        }

        writeln!(html, "</table>").unwrap();

        html
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_valid_slot() {
        let planner = WeekPlan::new(Time::new(8, 30).unwrap(), 90, 7).unwrap();
        assert!(planner.is_valid_slot(Time::new(14, 30).unwrap()));
        assert!(!planner.is_valid_slot(Time::new(14, 00).unwrap()));
    }
}
