use chrono::{DateTime, Datelike, Duration, FixedOffset, NaiveDate};

pub struct DateHelper;

const MONTHS_IN_YEAR: u8 = 12;

impl DateHelper {
    pub fn increment_date(date: &DateTime<FixedOffset>, increment: u8) -> DateTime<FixedOffset> {
        let mut new_date = date.clone();
        new_date += Duration::days(increment.into());

        if new_date.day() > Self::last_day_of_month(new_date.year(), new_date.month()).day() {
            new_date.with_day(increment.into());
            new_date.with_month(new_date.month() + increment as u32);

            if new_date.month() > MONTHS_IN_YEAR as u32 {
                new_date.with_year(new_date.year() + increment as i32);
                new_date.with_month(increment.into());
            }
        }

        return new_date;
    }
    pub fn last_day_of_month(year: i32, month: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(year, month + 1, 1)
            .unwrap_or(NaiveDate::from_ymd_opt(year + 1, 1, 1).unwrap())
            .pred_opt()
            .unwrap()
    }
}
