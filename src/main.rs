#![feature(plugin, decl_macro)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate premium_friday;

use premium_friday::*;

#[get("/")]
fn index() -> &'static str {
    "
    USAGE
      GET /<year>/<month>/<day>
    "
}

#[get("/<year>/<month>/<day>")]
fn ask(year: i32, month: u32, day: u32) -> Option<String> {
    let p = PremiumFriday::new().set_start_date(2017, 2, 24);
    p.is_premium_friday(year, month, day).map(|result| format!("{}", result))
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
