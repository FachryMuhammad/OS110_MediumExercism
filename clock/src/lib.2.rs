use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Clock {
    hour: i32,
    minute: i32,
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.hour, self.minute)
    }
}

impl Clock {
    pub fn new(hour: i32, minute: i32) -> Clock {
        let mut c = Clock { hour: 0, minute: 0 };
        c.add_hours(hour);
        c.add_minutes(minute)
    }

    /// Helper function to add hours with overflow.
    fn add_hours(&mut self, hours: i32) {
        self.hour += hours;

        // fix overflow
        self.hour %= 24;

        // fix underflow
        if self.hour < 0 {
            self.hour = 24 + self.hour;
        }
    }

    /// Add `minutes` to our clock. Can be negative or positive, and will adjust overflows
    /// including in the hour posotion.
    /// Consumes the clock to make borrow checker happy when doing Clock::new().add_minutes()
    pub fn add_minutes(mut self, minutes: i32) -> Clock {
        self.add_hours(minutes / 60);
        self.minute += minutes % 60;

        // overflow
        if self.minute > 60 {
            self.minute %= 60;
            self.add_hours(1);
        }

        // underflow
        if self.minute < 0 {
            self.minute = 60 + self.minute;
            self.add_hours(-1);
        }
        self
    }
}