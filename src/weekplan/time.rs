use std::fmt;
use std::{cmp::Ordering, ops::Deref};

use serde::de::{Deserialize, Deserializer};
use serde::ser::{Serialize, SerializeStruct};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Time {
    hour: Hour,
    minute: Minute,
}

mod time_impl {
    use super::Time;
    use serde::de::{self, Deserialize, Deserializer, MapAccess, Visitor};
    use std::fmt;

    enum Field {
        Hour,
        Minute,
    }

    // The Visitor for Time deserialization
    pub struct TimeVisitor;

    impl<'de> Visitor<'de> for TimeVisitor {
        type Value = Time;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("struct Time")
        }

        fn visit_map<V>(self, mut map: V) -> Result<Time, V::Error>
        where
            V: MapAccess<'de>,
        {
            let mut hour = None;
            let mut minute = None;

            while let Some(key) = map.next_key()? {
                match key {
                    Field::Hour => {
                        if hour.is_some() {
                            return Err(de::Error::duplicate_field("hour"));
                        }
                        hour = Some(map.next_value()?);
                    }
                    Field::Minute => {
                        if minute.is_some() {
                            return Err(de::Error::duplicate_field("minute"));
                        }
                        minute = Some(map.next_value()?);
                    }
                }
            }

            let hour = hour.ok_or_else(|| de::Error::missing_field("hour"))?;
            let minute = minute.ok_or_else(|| de::Error::missing_field("minute"))?;
            Time::new(hour, minute).ok_or_else(|| de::Error::custom("Invalid time"))
        }
    }

    // Field enum deserialization helper
    impl<'de> Deserialize<'de> for Field {
        fn deserialize<D>(deserializer: D) -> Result<Field, D::Error>
        where
            D: Deserializer<'de>,
        {
            struct FieldVisitor;

            impl<'de> Visitor<'de> for FieldVisitor {
                type Value = Field;

                fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                    formatter.write_str("`hour` or `minute`")
                }

                fn visit_str<E>(self, value: &str) -> Result<Field, E>
                where
                    E: de::Error,
                {
                    match value {
                        "hour" => Ok(Field::Hour),
                        "minute" => Ok(Field::Minute),
                        _ => Err(de::Error::unknown_field(value, &["hour", "minute"])),
                    }
                }
            }

            deserializer.deserialize_identifier(FieldVisitor)
        }
    }
}

impl<'de> Deserialize<'de> for Time {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        const FIELDS: &'static [&'static str] = &["hour", "minute"];
        deserializer.deserialize_struct("Time", FIELDS, time_impl::TimeVisitor)
    }
}

impl Serialize for Time {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut s = serializer.serialize_struct("Time", 2)?;
        s.serialize_field("hour", &*self.hour)?;
        s.serialize_field("minute", &*self.minute)?;
        s.end()
    }
}

impl fmt::Display for Time {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.hour(), self.minute())
    }
}

impl Time {
    pub fn new(hour: u8, minute: u8) -> Option<Self> {
        let hour = Hour::new(hour)?;
        let minute = Minute::new(minute)?;

        Some(Self { hour, minute })
    }

    fn unpack(&self) -> (u8, u8) {
        (*self.hour, *self.minute)
    }

    pub fn to_minutes(&self) -> u16 {
        let (hour, minute) = self.unpack();
        u16::from(minute) + u16::from(hour) * 60
    }

    pub fn try_sum(&self, duration: u16) -> Option<Self> {
        let (hour, minute) = self.unpack();

        let minute = u16::from(minute) + duration;

        let hour = hour + u8::try_from(minute / 60).ok()?;
        let minute = u8::try_from(minute % 60).unwrap();

        Time::new(hour, minute)
    }

    pub fn hour(&self) -> u8 {
        *self.hour
    }

    pub fn minute(&self) -> u8 {
        *self.minute
    }
}

impl PartialOrd for Time {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.hour.partial_cmp(&other.hour) {
            Some(Ordering::Equal) => self.minute.partial_cmp(&other.minute),
            ordering => ordering,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Hour(u8);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Minute(u8);

impl Hour {
    fn new(hour: u8) -> Option<Self> {
        if hour < 24 {
            Some(Self(hour))
        } else {
            None
        }
    }
}

impl Deref for Hour {
    type Target = u8;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Minute {
    fn new(minute: u8) -> Option<Self> {
        if minute < 60 {
            Some(Self(minute))
        } else {
            None
        }
    }
}

impl Deref for Minute {
    type Target = u8;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
