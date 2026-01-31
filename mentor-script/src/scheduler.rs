use std::arch::aarch64::int64x1x3_t;
use chrono::{DateTime, Local, Timelike};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CheckType {
    HalfHour,
    Hour,
}

pub fn check_time() -> Option<CheckType> {
    let now = chrono::Local::now();
    let minute = now.minute();

    match minute {
        30 => Some(CheckType::HalfHour),
        55 => Some(CheckType::Hour),
        _ => None,
    }

}

pub fn minutes_until_next_check(now: DateTime<Local>) -> (CheckType, i64) {
    let m = now.minute();
    if m < 30 {
        (CheckType::HalfHour, (30 - m) as i64)
    } else if m < 55 {
        (CheckType::Hour, (55 - m) as i64)
    } else {
        (CheckType::HalfHour, (60 - m + 30) as i64)
    }
}
