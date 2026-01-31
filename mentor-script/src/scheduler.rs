use chrono::Timelike;

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
