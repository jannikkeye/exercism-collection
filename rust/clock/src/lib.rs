use std::fmt;

#[derive(Debug, Eq, PartialEq)]
pub struct Clock {
    hours: u8,
    minutes: u8,
}

fn resolve_minutes(minutes: i32) -> (i32, i32) {
    let mut resolved_minutes = minutes % 60;
    let mut resolved_hours = minutes / 60;

    if minutes > 0 && resolved_minutes == 0 {
        resolved_minutes = 0;
    } else if minutes < 0 && resolved_minutes != 0 {
        resolved_hours -= 1;
        resolved_minutes += 60;
    }

    (resolved_hours, resolved_minutes)
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let (resolved_hours, resolved_minutes) = resolve_minutes(minutes);
        let hours = (hours + resolved_hours) % 24;



        Clock {
            hours: if hours < 0 { hours + 24 } else { hours } as u8,
            minutes: if resolved_minutes < 0 { resolved_minutes + 60 } else { resolved_minutes } as u8,
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        if minutes == 0 {
            return Self::new(self.hours as i32, self.minutes as i32);
        }

        let (resolved_hours, resolved_minutes) = resolve_minutes(minutes + self.minutes as i32);

        let hours = (self.hours as i32 + resolved_hours) % 24;

        Self::new(hours, resolved_minutes)
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}:{}",
            if self.hours < 10 { format!("0{}", self.hours) } else { self.hours.to_string() },
            if self.minutes < 10 { format!("0{}", self.minutes) } else { self.minutes.to_string() },
        )
    }
}