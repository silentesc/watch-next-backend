use chrono::{NaiveDateTime, Utc};
use time::OffsetDateTime;

pub fn utc_now_naive() -> NaiveDateTime {
    Utc::now().naive_utc()
}

pub fn utc_now_offset() -> OffsetDateTime {
    OffsetDateTime::now_utc()
}
