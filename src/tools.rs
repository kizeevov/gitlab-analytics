use chrono::{DateTime, Datelike, Duration, TimeZone, Weekday};
use std::mem;

pub trait DateTimeExtension {
    fn duration_since_without_weekend<Tz2: TimeZone>(self, rhs: DateTime<Tz2>) -> Duration;
}

impl<Tz: TimeZone> DateTimeExtension for DateTime<Tz> {
    fn duration_since_without_weekend<Tz2: TimeZone>(self, rhs: DateTime<Tz2>) -> Duration {
        let hours = DateTimeRange(rhs, self, Duration::hours(1))
            .into_iter()
            .filter(|x| x.date().weekday() != Weekday::Sat || x.date().weekday() != Weekday::Sun)
            .count() as i64;
        Duration::hours(hours)
    }
}

struct DateTimeRange<Tz: TimeZone, Tz2: TimeZone>(DateTime<Tz>, DateTime<Tz2>, Duration);

impl<Tz: TimeZone, Tz2: TimeZone> Iterator for DateTimeRange<Tz, Tz2> {
    type Item = DateTime<Tz>;
    fn next(&mut self) -> Option<Self::Item> {
        if &self.0 <= &self.1 {
            let next = self.0.clone() + self.2;
            Some(mem::replace(&mut self.0, next))
        } else {
            None
        }
    }
}
