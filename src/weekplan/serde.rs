use std::collections::HashMap;
use std::fmt;

use serde::de::{self, Deserializer, MapAccess, Visitor};
use serde::ser::{Serialize, SerializeStruct, Serializer};
use serde::Deserialize;

use crate::weekplan::{Time, Weekday};

use super::WeekPlan;

impl Serialize for WeekPlan {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("WeekPlan", 4)?;
        s.serialize_field("plan", &self.plan)?;
        s.serialize_field("start", &self.start)?;
        s.serialize_field("slot_duration", &self.slot_duration)?;
        s.serialize_field("slots", &self.slots)?;
        s.end()
    }
}

impl<'de> Deserialize<'de> for WeekPlan {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(field_identifier, rename_all = "snake_case")]
        enum Field {
            Plan,
            Start,
            SlotDuration,
            Slots,
        }

        struct WeekPlanVisitor;

        impl<'de> Visitor<'de> for WeekPlanVisitor {
            type Value = WeekPlan;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct WeekPlan")
            }

            fn visit_map<V>(self, mut map: V) -> Result<WeekPlan, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut plan: Option<HashMap<(Weekday, Time), String>> = None;
                let mut start: Option<Time> = None;
                let mut slot_duration: Option<u16> = None;
                let mut slots: Option<u8> = None;

                while let Some(key) = map.next_key()? {
                    match key {
                        Field::Plan => {
                            if plan.is_some() {
                                return Err(de::Error::duplicate_field("plan"));
                            }
                            plan = Some(map.next_value()?);
                        }
                        Field::Start => {
                            if start.is_some() {
                                return Err(de::Error::duplicate_field("start"));
                            }
                            start = Some(map.next_value()?);
                        }
                        Field::SlotDuration => {
                            if slot_duration.is_some() {
                                return Err(de::Error::duplicate_field("slot_duration"));
                            }
                            slot_duration = Some(map.next_value()?);
                        }
                        Field::Slots => {
                            if slots.is_some() {
                                return Err(de::Error::duplicate_field("slots"));
                            }
                            slots = Some(map.next_value()?);
                        }
                    }
                }

                let plan = plan.ok_or_else(|| de::Error::missing_field("plan"))?;
                let start = start.ok_or_else(|| de::Error::missing_field("start"))?;
                let slot_duration =
                    slot_duration.ok_or_else(|| de::Error::missing_field("slot_duration"))?;
                let slots = slots.ok_or_else(|| de::Error::missing_field("slots"))?;

                let mut week_plan = WeekPlan::new(start, slot_duration, slots)
                    .ok_or_else(|| de::Error::custom("Invalid WeekPlan"))?;

                for ((weekday, slot), activity) in plan {
                    week_plan
                        .try_insert(weekday, slot, activity)
                        .map_err(|err| {
                            de::Error::custom(format!("Invalid slot {slot} for activity: {err:?}"))
                        })?;
                }

                Ok(week_plan)
            }
        }

        const FIELDS: &'static [&'static str] = &["plan", "start", "slot_duration", "slots"];
        deserializer.deserialize_struct("WeekPlan", FIELDS, WeekPlanVisitor)
    }
}

#[cfg(test)]
mod test {
    use crate::weekplan::*;

    #[test]
    fn weekday() {
        let day_in = Weekday::Monday;
        let json_out = serde_json::to_string(&day_in).unwrap();

        let json_in = r#""Monday""#;
        let day_out: Weekday = serde_json::from_str(&json_in).unwrap();

        assert_eq!(day_in, day_out);
        assert_eq!(json_in, json_out);
    }

    #[test]
    fn time() {
        let time_in = Time::new(8, 30).unwrap();
        let json_out = serde_json::to_string(&time_in).unwrap();

        let json_in = r#"{"hour":8,"minute":30}"#;
        let time_out: Time = serde_json::from_str(&r#"{"hour": 8, "minute": 30}"#).unwrap();

        assert_eq!(time_in, time_out);
        assert_eq!(json_in, json_out);
    }

    #[test]
    fn weekplan() {
        let mut week_plan_in = WeekPlan::new(Time::new(8, 30).unwrap(), 90, 7).unwrap();
        week_plan_in
            .try_insert(Weekday::Monday, Time::new(8, 30).unwrap(), "AAA".into())
            .unwrap()
            .try_insert(Weekday::Tuesday, Time::new(10, 00).unwrap(), "BBB".into())
            .unwrap();

        let week_plan_in = week_plan_in;
        let json_out = serde_json::to_string(&week_plan_in).unwrap();

        println!("{json_out}");
    }
}
