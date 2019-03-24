use chrono::prelude::*;
use http::{self, StatusCode};
use now_lambda::{lambda, IntoResponse, Request, Response};
use premium_friday::*;
use serde::Serialize;
use std::error::Error;

#[derive(Serialize, Debug)]
struct ResultJson {
    #[serde(with = "fixed_offset_format")]
    date: DateTime<FixedOffset>,
    result: bool,
}

#[derive(Serialize, Debug)]
struct ErrJson {
    error: String,
}

mod fixed_offset_format {
    use chrono::{DateTime, FixedOffset};
    use serde::Serializer;

    pub fn serialize<S>(date: &DateTime<FixedOffset>, serializer: S) -> Result<S::Ok, S::Error>
    where S: Serializer,
    {
        serializer.serialize_str(&date.to_rfc3339())
    }
}

fn handler(request: Request) -> Result<impl IntoResponse, http::Error> {
    let premium_friday = PremiumFriday::default().set_start_date(2017, 2, 24);

    let (status_code, body) = match request.uri().path() {
        "/today" => today(premium_friday, get_today()),
        "/json" => json(premium_friday, get_today()),
        _ => not_found(),
    };

    Response::builder().status(status_code).body(body)
}

fn get_today() -> DateTime<FixedOffset> {
    let utc_now = Utc::now();
    let tz_offset = FixedOffset::east(9 * 3600);

    utc_now.with_timezone(&tz_offset)
}

fn today(premium_friday: PremiumFriday, date: DateTime<FixedOffset>) -> (StatusCode, String) {
    let result = premium_friday
        .is_premium_friday(date.year(), date.month(), date.day())
        .unwrap_or(false);

    (StatusCode::OK, result.to_string())
}

fn json(premium_friday: PremiumFriday, date: DateTime<FixedOffset>) -> (StatusCode, String) {
    let result = premium_friday
        .is_premium_friday(date.year(), date.month(), date.day())
        .unwrap_or(false);
    let value = ResultJson { date, result };
    let serialized = serde_json::to_string(&value);

    match serialized {
        Ok(json) => (StatusCode::OK, json),
        Err(err) => (StatusCode::INTERNAL_SERVER_ERROR, serde_json::to_string(&ErrJson { error: err.to_string() }).unwrap()),
    }
}

fn not_found() -> (StatusCode, String) {
    let usage = "\
                 USAGE\
                 \n    GET /<year>/<month>/<day>\
                 \n    GET /today\
                 \n    GET /json\
                 ";

    (StatusCode::OK, usage.to_string())
}

fn main() -> Result<(), Box<dyn Error>> {
    Ok(lambda!(handler))
}
