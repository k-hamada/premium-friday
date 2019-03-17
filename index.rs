extern crate chrono;
extern crate premium_friday;

use chrono::prelude::*;
use premium_friday::*;
use http::{self, Request, Response, StatusCode};

fn handler(_request: Request<()>) -> http::Result<Response<String>> {
    let premium_friday = PremiumFriday::default().set_start_date(2017, 2, 24);

    let today = get_today();
    match premium_friday.is_premium_friday(today.year(), today.month(), today.day()) {
        Some(result) => output(result),
        None => not_found()
    }
}

fn get_today() -> DateTime<FixedOffset> {
    let utc_now = Utc::now();
    let tz_offset = FixedOffset::east(9 * 3600);

    utc_now.with_timezone(&tz_offset)
}

fn output(is_premium_friday: bool) -> http::Result<Response<String>> {
    Response::builder()
        .status(StatusCode::OK)
        .body(is_premium_friday.to_string())
}

fn not_found() -> http::Result<Response<String>> {
    Response::builder()
        .status(StatusCode::OK)
        .body("
        USAGE
            GET /<year>/<month>/<day>
            GET /today
            GET /json
        ".to_string())
}
