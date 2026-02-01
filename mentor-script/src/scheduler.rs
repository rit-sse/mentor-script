//! Scheduling logic for reminder timing
//!
//! Determines when reminders should trigger and calculates time until next check.

use std::fmt;
use std::fmt::Formatter;
use chrono::{DateTime, Local, Timelike};

/// Type of check-in reminder
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CheckType {
    /// 30-minute check (triggers at :30)
    HalfHour,
    /// Hourly check (triggers at :55)
    Hour,
}

impl fmt::Display for CheckType {
   /// Implements display output for each CheckType for better formatting.
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            CheckType::Hour => write!(f, "One hour check"),
            CheckType::HalfHour => write!(f, "Half hour check"),
        }
    }
}

/// Returns the type of check if the current time matches a reminder trigger
///
/// Triggers at minute :30 (HalfHour) and :55 (Hour)
pub fn check_time() -> Option<CheckType> {
    let now = Local::now();
    let minute = now.minute();

    match minute {
        30 => Some(CheckType::HalfHour),
        55 => Some(CheckType::Hour),
        _ => None,
    }

}

/// Calculates which check is next and how many minutes until it triggers
///
/// Returns (CheckType, minutes_remaining)
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
