use std::fmt;
use std::str::FromStr;
use std::{cmp::Ordering, ops::Deref};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Time {
    hour: Hour,
    minute: Minute,
}

#[derive(Debug)]
pub enum ParseTimeError {
    BadFormat,
    InvalidHour,
    InvalidMinute,
    BreaksInvariant,
}

impl fmt::Display for ParseTimeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let msg = match self {
            ParseTimeError::BadFormat => "bad format",
            ParseTimeError::InvalidHour => "invalid hour",
            ParseTimeError::InvalidMinute => "invalid minute",
            ParseTimeError::BreaksInvariant => "time doesn't respect the invariants for Time",
        };

        write!(f, "{msg}")
    }
}

impl std::error::Error for ParseTimeError {}

impl fmt::Display for Time {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.hour(), self.minute())
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

impl FromStr for Time {
    type Err = ParseTimeError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(":").map(|s| s.trim()).collect();

        if parts.len() != 2 {
            return Err(ParseTimeError::BadFormat);
        }

        let hour = unsafe { *parts.get_unchecked(0) };
        let minute = unsafe { *parts.get_unchecked(1) };

        let hour = u8::from_str(hour).map_err(|_| ParseTimeError::InvalidHour)?;
        let minute = u8::from_str(minute).map_err(|_| ParseTimeError::InvalidMinute)?;

        Time::new(hour, minute).ok_or(ParseTimeError::BreaksInvariant)
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
