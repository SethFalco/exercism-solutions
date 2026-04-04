use core::fmt;

const DAY_TO_MINUTES: i16 = 1440;
const HOUR_TO_MINUTES: i16 = 60;

#[derive(PartialEq, Debug)]
pub struct Clock {
    /// Minutes since the start of the current day, value between 0 and 1439.
    day_epoch: i16
}

impl Clock {

    pub fn new(hours: i16, minutes: i16) -> Self {
        let instance = Self {
            day_epoch: 0
        };

        instance.add_minutes(hours * HOUR_TO_MINUTES + minutes)
    }

    pub fn add_minutes(&self, mut minutes: i16) -> Self {
        while minutes < 0 {
            minutes += DAY_TO_MINUTES;
        }

        Self {
            day_epoch: (self.day_epoch + minutes) % DAY_TO_MINUTES
        }
    }
}

impl fmt::Display for Clock {

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let hours = self.day_epoch / HOUR_TO_MINUTES;
        let minutes = self.day_epoch - hours * HOUR_TO_MINUTES;

        write!(f, "{:0>2}:{:0>2}", hours, minutes)
    }
}
