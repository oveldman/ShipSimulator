use chrono::{ DateTime, Duration, Utc };

pub fn add_time(current_time: DateTime<Utc>, add_days: i64) -> DateTime<Utc> {
    current_time + Duration::days(add_days)
}