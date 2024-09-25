use std::collections::HashMap;

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
