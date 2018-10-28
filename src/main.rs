#[macro_use]
extern crate warp;
extern crate chrono;
extern crate premium_friday;
extern crate pretty_env_logger;
extern crate serde;
#[macro_use]
extern crate serde_derive;

use warp::Filter;
use std::sync::Arc;
use chrono::prelude::*;
use premium_friday::*;

#[derive(Serialize, Debug)]
struct Json {
    today: bool,
}

fn index() -> impl warp::Reply {
    "
    USAGE
      GET /<year>/<month>/<day>
      GET /today
      GET /json
    "
}

fn ask(year: i32, month: u32, day: u32, p: Arc<PremiumFriday>)
    -> Result<impl warp::Reply, warp::Rejection>
{
    p.is_premium_friday(year, month, day)
        .map(|result| output(year, month, day, result))
        .ok_or(warp::reject::bad_request())
}

fn today(p: Arc<PremiumFriday>, (year, month, day): (i32, u32, u32))
    -> Result<impl warp::Reply, warp::Rejection>
{
    p.is_premium_friday(year, month, day)
        .map(|result| output(year, month, day, result))
        .ok_or(warp::reject::server_error())
}

fn json(p: Arc<PremiumFriday>, (year, month, day): (i32, u32, u32))
    -> Result<impl warp::Reply, warp::Rejection>
{
    p.is_premium_friday(year, month, day)
        .map(|result| warp::reply::json(&Json { today: result }))
        .ok_or(warp::reject::server_error())
}

fn output(year: i32, month: u32, day: u32, result: bool) -> String {
    let date = format!("{}/{}/{}", year, month, day);

    if result {
        format!("{} is PremiumFriday", date)
    } else {
        format!("{} is NOT PremiumFriday", date)
    }
}

fn get_today() -> DateTime<FixedOffset> {
    let utc_now = Utc::now();
    let tz_offset = FixedOffset::east(9 * 3600);

    utc_now.with_timezone(&tz_offset)
}

fn main() {
    pretty_env_logger::init();

    let premium_friday = PremiumFriday::new().set_start_date(2017, 2, 24);
    let premium_friday = Arc::new(premium_friday);
    let with_premium_friday = warp::any().map(move || premium_friday.clone());

    let with_today = warp::any().map(|| {
        let today = get_today();

        (today.year(), today.month(), today.day())
    });

    let index = warp::path::end()
        .map(index);

    let ask = path!(i32 / u32 / u32)
        .and(with_premium_friday.clone())
        .and_then(ask);

    let today = path!("today")
        .and(with_premium_friday.clone())
        .and(with_today)
        .and_then(today);

    let json = path!("json")
        .and(with_premium_friday.clone())
        .and(with_today)
        .and_then(json);

    let routes = warp::get2().and(
        index
            .or(ask)
            .or(today)
            .or(json)
    );

    warp::serve(routes)
        .run(([0, 0, 0, 0], 80));
}
