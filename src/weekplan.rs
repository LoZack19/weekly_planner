use core::fmt;
use std::fmt::Write;
use std::ops::SubAssign;
use std::{collections::HashMap, str::FromStr};

use ::serde::{de, Deserialize, Deserializer, Serialize};
pub use activity::Activity;
pub use time::Time;
pub use weekday::Weekday;

mod activity;
mod serde;
mod time;
mod weekday;

#[derive(Debug, Hash, PartialEq, Eq)]
struct Slot(Weekday, Time);

impl Serialize for Slot {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        format!("{:?} {}", self.0, self.1).serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Slot {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let mut parts = s.split_whitespace();

        let weekday = parts
            .next()
            .ok_or_else(|| de::Error::custom("Missing weekday"))?;
        let time = parts.next().ok_or_else(|| {
            let custom = de::Error::custom("Missing time");
            custom
        })?;

        let weekday =
            Weekday::from_str(weekday).map_err(|_| de::Error::custom("Invalid weekday"))?;
        let time = Time::from_str(time).map_err(|_| de::Error::custom("Invalid time"))?;

        Ok(Slot(weekday, time))
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct WeekPlan {
    plan: HashMap<Slot, Activity>,
    start: Time,
    slot_duration: u16,
    slots: u8,
}

#[derive(Debug)]
pub enum Error {
    InvalidSlot,
    AlreadyBooked,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let msg = match self {
            Error::InvalidSlot => "Invalid slot",
            Error::AlreadyBooked => "Already booked",
        };

        write!(f, "{msg}")
    }
}

impl std::error::Error for Error {}

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
        let distance = match slot.checked_sub(start) {
            Some(val) => val,
            None => {
                return false;
            }
        };

        let slot_index = match u8::try_from(distance / u16::from(self.slot_duration)) {
            Ok(val) => val,
            Err(_) => {
                return false;
            }
        };

        distance % u16::from(self.slot_duration) == 0 && slot_index < self.slots
    }

    pub fn try_insert(
        &mut self,
        weekday: Weekday,
        slot: Time,
        activity: Activity,
    ) -> Result<&mut Self> {
        if !self.is_valid_slot(slot) {
            return Err(Error::InvalidSlot);
        }

        let key = Slot(weekday, slot);

        if self.plan.contains_key(&key) {
            return Err(Error::AlreadyBooked);
        }

        self.plan.insert(key, activity);

        Ok(self)
    }

    pub fn try_insert_range(
        &mut self,
        weekday: Weekday,
        slots: (Time, u8),
        activity: Activity,
    ) -> Result<&mut Self> {
        let (start, len) = slots;

        for i in 0..len {
            self.try_insert(
                weekday,
                start
                    .try_sum(u16::from(i) * self.slot_duration)
                    .ok_or(Error::InvalidSlot)?,
                activity.clone(),
            )?;
        }

        Ok(self)
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
                    .map(|&time| {
                        self.plan
                            .get(&Slot(weekday, time))
                            .unwrap_or(&default_activity)
                    })
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
