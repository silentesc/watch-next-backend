use axum_extra::extract::cookie::{Cookie, SameSite};
use time::{Duration, OffsetDateTime};

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

pub fn session_expiration() -> OffsetDateTime {
    OffsetDateTime::now_utc() + Duration::days(7)
}
