use axum_extra::extract::cookie::{Cookie, SameSite};
use chrono::NaiveDateTime;
use time::OffsetDateTime;

use crate::api::utils::time_utils;

pub fn default_cookie<'a>(session_id: String, expires: OffsetDateTime) -> Cookie<'a> {
    Cookie::build(("session_id", session_id))
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
