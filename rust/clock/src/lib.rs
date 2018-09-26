use std::fmt;

#[derive(Debug, PartialEq)]
pub struct Clock {
    hours: i32,
    minutes: i32
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let (hours, minutes) = Clock::calc_roll_over(hours, minutes);
        Clock {
            hours,
            minutes
        }
    }

    // Remove duplicate code by calling Clock::new()
    // plus the added minutes here
    pub fn add_minutes(&self, add_minutes: i32) -> Self {
        Clock::new(self.hours, self.minutes + add_minutes)
    }  

    // making the roll over calculation a private method of Clock
    fn calc_roll_over(hours: i32, minutes: i32) -> (i32, i32) {
        let minutes_per_day = 24 * 60;
        let mut minutes = hours * 60 + minutes;

        minutes = (minutes % minutes_per_day + minutes_per_day) % minutes_per_day; 
        let hours = minutes / 60;
        minutes = minutes % 60;

        (hours, minutes)
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}