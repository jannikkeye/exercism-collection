use chrono::{DateTime, Duration, Utc};

const ONE_BILLION: i64 = 1_000_000_000;

pub fn after(start: DateTime<Utc>) -> DateTime<Utc> {
    start + Duration::seconds(ONE_BILLION)
}
