use core::fmt;
use std::{collections::HashMap, fmt::Display, str::FromStr};

use ::serde::{de, Deserialize, Deserializer, Serialize};
pub use activity::Activity;
pub use time::Time;
pub use weekday::Weekday;

mod activity;
mod serde;
mod time;
mod weekday;

#[derive(Debug, Hash, PartialEq, Eq)]
pub struct Slot(Weekday, Time);

impl Display for Slot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} {}", self.0, self.1.to_string())
    }
}

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
    InvalidSlot(Time),
    AlreadyBooked(Slot),
    OutOfBounds,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let msg = match self {
            Error::InvalidSlot(time) => format!("Invalid slot {time}"),
            Error::AlreadyBooked(slot) => format!("Slot {slot} already booked"),
            Error::OutOfBounds => "Slot is outside of the last hour for the day".to_owned(),
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
            return Err(Error::InvalidSlot(slot));
        }

        let key = Slot(weekday, slot);

        if self.plan.contains_key(&key) {
            return Err(Error::AlreadyBooked(Slot(weekday, slot)));
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
                    .ok_or(Error::OutOfBounds)?,
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
        let (weekdays, times, table) = self.to_table();

        let css = r#"
            body {
                font-family: Arial, sans-serif;
                display: flex;
                justify-content: center;
                align-items: center;
                min-height: 100vh;
                margin: 0;
                background-color: #f0f0f0;
            }
            .schedule-table {
                border-collapse: collapse;
                box-shadow: 0 0 20px rgba(0, 0, 0, 0.1);
                background-color: white;
            }
            .schedule-table th,
            .schedule-table td {
                padding: 12px 15px;
                text-align: center;
            }
            .schedule-table th {
                background-color: #009879;
                color: white;
                text-transform: uppercase;
                font-weight: bold;
            }
            .schedule-table td {
                border-bottom: 1px solid #dddddd;
            }
            .schedule-table tr:nth-child(even) {
                background-color: #f3f3f3;
            }
            .schedule-table tr:last-of-type {
                border-bottom: 2px solid #009879;
            }
            .schedule-table tr:hover {
                background-color: #f5f5f5;
                transition: background-color 0.3s ease;
            }
            .header-row th:first-child {
                background-color: #007965;
            }
        "#;

        let html_start = format!(
            r#"<!DOCTYPE html>
    <html lang="en">
    <head>
        <meta charset="UTF-8">
        <meta name="viewport" content="width=device-width, initial-scale=1.0">
        <title>Styled Table</title>
        <style>
        {}
        </style>
    </head>
    <body>
        <table class="schedule-table">
            <tr class="header-row">
                <th></th>"#,
            css
        );

        let html_end = r#"    </table>
    </body>
    </html>"#;

        let mut html = String::new();
        html.push_str(&html_start);

        // Add weekday headers
        for day in &weekdays {
            html.push_str(&format!("            <th>{:?}</th>\n", day));
        }
        html.push_str("        </tr>\n");

        // Add table rows
        for (i, time) in times.iter().enumerate() {
            html.push_str("        <tr>\n");
            html.push_str(&format!("            <th>{}</th>\n", time.to_string(),));

            for j in 0..weekdays.len() {
                let activity = &table[i + j * times.len()];
                html.push_str(&format!("            <td>{}</td>\n", activity));
            }

            html.push_str("        </tr>\n");
        }

        html.push_str(html_end);
        html
    }
}

#[macro_export]
macro_rules! poli_plan {
    ($start:expr, $duration:expr, $days:expr, $($day:expr => $time:expr , $length:expr , $name:expr),* $(,)?) => {{
        let mut w = WeekPlan::new($start, $duration, $days).unwrap();
        $(
            let weekday = Weekday::from_str($day).unwrap();
            let time = Time::from_str($time).unwrap();
            w.try_insert_range(
                weekday,
                (time, $length),
                $name.to_owned(),
            ).unwrap();
        )*
        w
    }};
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
