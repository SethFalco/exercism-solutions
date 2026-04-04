use core::fmt;

#[derive(PartialEq, Debug)]
pub struct Clock {
    /// Minutes since the start of the current day, value between 0 and 1439.
    day_epoch: u16
}

impl Clock {

    pub fn new(hours: i16, minutes: i16) -> Self {
        let instance = Self {
            day_epoch: 0
        };

        instance.add_minutes(hours * 60 + minutes)
    }

    pub fn add_minutes(&self, mut minutes: i16) -> Self {
        while minutes < 0 {
            minutes += 1440;
        }

        Self {
            day_epoch: (self.day_epoch + minutes as u16) % 1440
        }
    }
}

impl fmt::Display for Clock {

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let hours = self.day_epoch / 60;
        let minutes = self.day_epoch - hours * 60;

        write!(f, "{:0>2}:{:0>2}", hours, minutes)
    }
}
