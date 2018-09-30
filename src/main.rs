#![feature(plugin, decl_macro)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate chrono;
extern crate premium_friday;

use rocket::response::Redirect;
use chrono::prelude::*;
use premium_friday::*;

#[get("/")]
fn index() -> &'static str {
    "
    USAGE
      GET /<year>/<month>/<day>
      GET /today
    "
}

#[get("/<year>/<month>/<day>")]
fn ask(year: i32, month: u32, day: u32) -> Option<String> {
    let p = PremiumFriday::new().set_start_date(2017, 2, 24);
    p.is_premium_friday(year, month, day).map(|result| format!("{}", result))
}

#[get("/today")]
fn today() -> Redirect  {
    let utc_today = Utc::today().naive_utc();
    let tz_offset = FixedOffset::east(9 * 3600);
    let today = tz_offset.from_utc_date(&utc_today);
    Redirect::to(format!("/{}/{}/{}", today.year(), today.month(), today.day()))
}

fn main() {
    rocket::ignite()
        .mount("/", routes![
            index,
            ask,
            today
        ])
        .launch();
}
