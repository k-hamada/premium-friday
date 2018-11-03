extern crate chrono;

use chrono::{Datelike, Duration, NaiveDate, Weekday};

#[derive(Default)]
pub struct PremiumFriday {
    start_date: Option<NaiveDate>,
    end_date: Option<NaiveDate>,
}

impl PremiumFriday {
    pub fn set_start_date(self, year: i32, month: u32, day: u32) -> Self {
        let start_date = NaiveDate::from_ymd_opt(year, month, day);

        PremiumFriday { start_date, ..self }
    }

    pub fn set_end_date(self, year: i32, month: u32, day: u32) -> Self {
        let end_date = NaiveDate::from_ymd_opt(year, month, day);

        PremiumFriday { end_date, ..self }
    }

    pub fn is_premium_friday(&self, year: i32, month: u32, day: u32) -> Option<bool> {
        NaiveDate::from_ymd_opt(year, month, day)
            .map(|date| self.is_friday(date) && self.is_last_weekday(date) && self.is_include_range(date))
    }

    fn is_friday(&self, date: NaiveDate) -> bool {
        date.weekday() == Weekday::Fri
    }

    fn is_last_weekday(&self, date: NaiveDate) -> bool {
        date.month() != (date + Duration::days(7)).month()
    }

    fn is_include_range(&self, date: NaiveDate) -> bool {
        self.start_date.map_or(true, |start_date| start_date <= date) &&
        self.end_date.map_or(true, |end_date| date < end_date)
    }
}
