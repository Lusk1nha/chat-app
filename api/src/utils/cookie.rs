use axum_extra::extract::{cookie::Cookie, CookieJar};

use super::session::REFRESH_EXPIRES_AT;

pub fn get_refresh_token_cookie(token: String, jar: CookieJar) -> CookieJar {
    let expires = time::OffsetDateTime::now_utc() + time::Duration::days(REFRESH_EXPIRES_AT);
    let max_age = time::Duration::days(REFRESH_EXPIRES_AT);

    let refresh_cookie = Cookie::build(("refresh_token", token.clone()))
        .path("/")
        .secure(true)
        .http_only(true)
        .max_age(max_age)
        .expires(expires)
        .build();

    let jar = jar.add(refresh_cookie);

    jar
}

pub fn clear_refresh_token_cookie(jar: CookieJar) -> CookieJar {
    let max_age = time::Duration::seconds(0);

    let refresh_cookie = Cookie::build(("refresh_token", ""))
        .path("/")
        .http_only(true)
        .secure(true)
        .max_age(max_age)
        .build();

    let jar = jar.add(refresh_cookie);

    jar
}
