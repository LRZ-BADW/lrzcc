use chrono::{DateTime, TimeZone, Utc};

pub fn e400<T>(e: T) -> actix_web::Error
where
    T: std::fmt::Debug + std::fmt::Display + 'static,
{
    actix_web::error::ErrorBadRequest(e)
}

pub fn e500<T>(e: T) -> actix_web::Error
where
    T: std::fmt::Debug + std::fmt::Display + 'static,
{
    actix_web::error::ErrorInternalServerError(e)
}

pub fn start_of_the_year(year: u32) -> DateTime<Utc> {
    // TODO: handle this unwrap
    Utc.with_ymd_and_hms(year as i32, 1, 1, 1, 0, 0).unwrap()
}
