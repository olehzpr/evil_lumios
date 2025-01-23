pub mod commands;
pub mod external;
pub mod schedule;
use std::ops::{Add, Mul};

use chrono::Datelike;

pub type HandlerResult = anyhow::Result<(), Box<dyn std::error::Error + Send + Sync>>;

#[derive(Copy, Clone, Debug)]
pub enum Day {
    Mon,
    Tue,
    Wed,
    Thu,
    Fri,
    Sat,
    Sun,
}

#[derive(Copy, Clone, Debug)]
pub enum Week {
    First,
    Second,
}

impl From<Day> for u8 {
    fn from(day: Day) -> Self {
        match day {
            Day::Mon => 0,
            Day::Tue => 1,
            Day::Wed => 2,
            Day::Thu => 3,
            Day::Fri => 4,
            Day::Sat => 5,
            Day::Sun => 6,
        }
    }
}

impl From<chrono::Weekday> for Day {
    fn from(day: chrono::Weekday) -> Self {
        match day {
            chrono::Weekday::Mon => Day::Mon,
            chrono::Weekday::Tue => Day::Tue,
            chrono::Weekday::Wed => Day::Wed,
            chrono::Weekday::Thu => Day::Thu,
            chrono::Weekday::Fri => Day::Fri,
            chrono::Weekday::Sat => Day::Sat,
            chrono::Weekday::Sun => Day::Sun,
        }
    }
}

impl From<Day> for &'static str {
    fn from(day: Day) -> Self {
        match day {
            Day::Mon => "Понеділок",
            Day::Tue => "Вівторок",
            Day::Wed => "Середа",
            Day::Thu => "Четвер",
            Day::Fri => "П'ятниця",
            Day::Sat => "Субота",
            Day::Sun => "Неділя",
        }
    }
}

impl Day {
    pub fn next(&self) -> Self {
        match self {
            Day::Mon => Day::Tue,
            Day::Tue => Day::Wed,
            Day::Wed => Day::Thu,
            Day::Thu => Day::Fri,
            Day::Fri => Day::Sat,
            Day::Sat => Day::Sun,
            Day::Sun => Day::Mon,
        }
    }
    pub fn current() -> Self {
        let now = chrono::Utc::now();
        let day = now.date_naive().weekday();
        day.into()
    }
}

impl From<Week> for u8 {
    fn from(week: Week) -> Self {
        match week {
            Week::First => 1,
            Week::Second => 2,
        }
    }
}

impl From<Week> for &str {
    fn from(week: Week) -> Self {
        match week {
            Week::First => "Тиждень 1",
            Week::Second => "Тиждень 2",
        }
    }
}

impl Week {
    pub fn next(&self) -> Self {
        match self {
            Week::First => Week::Second,
            Week::Second => Week::First,
        }
    }
    pub fn current() -> Self {
        let now = chrono::Utc::now();
        let start = chrono::NaiveDate::from_isoywd_opt(now.year(), 1, chrono::Weekday::Mon);
        let week_number = now.date_naive().iso_week().week() - start.unwrap().iso_week().week() + 1;
        match week_number % 2 {
            0 => Week::Second,
            _ => Week::First,
        }
    }
}

impl Add<u8> for Week {
    type Output = u8;

    fn add(self, rhs: u8) -> u8 {
        u8::from(self) + rhs
    }
}

impl Add<u8> for Day {
    type Output = u8;

    fn add(self, rhs: u8) -> u8 {
        u8::from(self) + rhs
    }
}

impl Mul<u8> for Week {
    type Output = u8;

    fn mul(self, rhs: u8) -> u8 {
        u8::from(self) * rhs
    }
}

impl Mul<u8> for Day {
    type Output = u8;

    fn mul(self, rhs: u8) -> u8 {
        u8::from(self) * rhs
    }
}

impl PartialEq for Day {
    fn eq(&self, other: &Self) -> bool {
        u8::from(*self) == u8::from(*other)
    }
}

impl PartialEq for Week {
    fn eq(&self, other: &Self) -> bool {
        u8::from(*self) == u8::from(*other)
    }
}
