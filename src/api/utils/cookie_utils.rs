use axum_extra::extract::cookie::{Cookie, SameSite};
use chrono::NaiveDateTime;
use time::{Duration, OffsetDateTime};

use crate::api::utils::time_utils;

pub const SESSION_ID_COOKIE_NAME: &str = "session_id";

pub fn removal_cookie<'a>(cookie_name: String) -> Cookie<'a> {
    Cookie::build((cookie_name, ""))
        .path("/")
        .max_age(Duration::ZERO)
        .build()
}

pub fn default_cookie<'a>(session_id: String, expires: OffsetDateTime) -> Cookie<'a> {
    Cookie::build((SESSION_ID_COOKIE_NAME, session_id))
        .http_only(true)
        .path("/")
        .same_site(SameSite::Strict)
        .expires(expires)
        .secure(true)
        .build()
}

pub fn expires_at_naive() -> NaiveDateTime {
    time_utils::utc_now_naive() + chrono::Duration::days(7)
}

pub fn expires_at_offset() -> OffsetDateTime {
    time_utils::utc_now_offset() + time::Duration::days(7)
}
