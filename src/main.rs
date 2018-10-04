#[macro_use]
extern crate warp;
extern crate chrono;
extern crate premium_friday;
extern crate pretty_env_logger;

use chrono::prelude::*;
use premium_friday::*;
use warp::Filter;

fn index() -> &'static str {
    "
    USAGE
      GET /<year>/<month>/<day>
      GET /today
    "
}

fn ask(year: i32, month: u32, day: u32) -> Result<String, warp::Rejection> {
    is_premium_friday(year, month, day)
        .ok_or(warp::reject::bad_request())
}

fn today() -> Result<String, warp::Rejection> {
    let utc_now = Utc::now();
    let tz_offset = FixedOffset::east(9 * 3600);
    let local_now = utc_now.with_timezone(&tz_offset);

    is_premium_friday(local_now.year(), local_now.month(), local_now.day())
        .ok_or(warp::reject::server_error())
}

fn is_premium_friday(year: i32, month: u32, day: u32) -> Option<String> {
    let p = PremiumFriday::new().set_start_date(2017, 2, 24);
    p.is_premium_friday(year, month, day)
        .map(|result| output(year, month, day, result))
}

fn output(year: i32, month: u32, day: u32, result: bool) -> String {
    let date = format!("{}/{}/{}", year, month, day);

    if result {
        format!("{} is PremiumFriday", date)
    } else {
        format!("{} is NOT PremiumFriday", date)
    }
}

fn main() {
    pretty_env_logger::init();

    let routes = warp::get2().and(
        warp::path::index().map(index)
            .or(path!(i32 / u32 / u32).and_then(ask))
            .or(path!("today").and_then(today))
    );

    warp::serve(routes)
        .run(([0, 0, 0, 0], 80));
}
