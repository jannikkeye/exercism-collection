#![feature(euclidean_division)]

use std::fmt;

#[derive(Debug, Eq)]
pub struct Clock {
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        Clock {
            minutes: minutes + hours * 60,
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
       Clock {
           minutes: self.minutes + minutes,
       }
    }
}

impl PartialEq for Clock {
    fn eq(&self, other: &Clock) -> bool {
        self.to_string() == other.to_string()
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut hours = (self.minutes / 60).rem_euclid(24);
        let mut minutes = self.minutes % 60;

        if minutes < 0 { 
            hours -= 1;

            if hours < 0 {
                hours = 24 + hours;
            } 
        }

        if minutes < 0 {
            minutes = 60 + minutes;
        }

        write!(
            f,
            "{:02}:{:02}",
            hours,
            minutes
        )
    }
}