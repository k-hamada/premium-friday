#![feature(plugin, decl_macro)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate chrono;
extern crate premium_friday;

use chrono::prelude::*;
use premium_friday::*;

#[get("/")]
fn index() -> &'static str {
    "
    USAGE
      GET /<year>/<month>/<day>
      GET /today
      GET /why
    "
}

#[get("/<year>/<month>/<day>")]
fn ask(year: i32, month: u32, day: u32) -> Option<String> {
    is_premium_friday(year, month, day)
}

#[get("/today")]
fn today() -> Option<String> {
    let utc_now = Utc::now();
    let tz_offset = FixedOffset::east(9 * 3600);
    let local_now = utc_now.with_timezone(&tz_offset);

    is_premium_friday(local_now.year(), local_now.month(), local_now.day())
}

fn is_premium_friday(year: i32, month: u32, day: u32) -> Option<String> {
    let p = PremiumFriday::new().set_start_date(2017, 2, 24);
    p.is_premium_friday(year, month, day).map(|result| output(year, month, day, result))
}

fn output(year: i32, month: u32, day: u32, result: bool) -> String {
    let date = format!("{}/{}/{}", year, month, day);

    if result {
        format!("{} is PremiumFriday", date)
    } else {
        format!("{} is NOT PremiumFriday", date)
    }
}

#[get("/why")]
fn why() -> &'static str {
    "
    I don't like premium friday
    "
}

fn main() {
    rocket::ignite()
        .mount("/", routes![
            index,
            ask,
            today,
            why
        ])
        .launch();
}
