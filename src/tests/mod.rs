use crate::DeviceConfig;
use chrono::{Datelike, Duration, NaiveDate, NaiveDateTime, NaiveTime, TimeZone, Timelike};

mod absolute_times;
mod mixed_absolute_relative_times;
mod relative_times;
mod window_parsing;

fn make_time_with_offset(hour: u32, min: u32, offset_seconds: i64) -> NaiveDateTime {
    NaiveDateTime::new(
        NaiveDate::from_ymd_opt(2024, 1, 2).unwrap(),
        NaiveTime::from_hms_opt(hour, min, 0).unwrap(),
    ) - Duration::seconds(offset_seconds)
}
const NZ_SUMMER_UTC_OFFSET_SECONDS: i64 = 13 * 60 * 60;
const NZ_WINTER_UTC_OFFSET_SECONDS: i64 = 12 * 60 * 60;
const UK_SUMMER_UTC_OFFSET_SECONDS: i64 = 1 * 60 * 60;
const UK_WINTER_UTC_OFFSET_SECONDS: i64 = 0 * 60 * 60;
const CAYMAN_ISLANDS_UTC_OFFSET_SECONDS: i64 = -5 * 60 * 60;
const NORFOLK_ISLAND_SUMMER_UTC_OFFSET_SECONDS: i64 = 12 * 60 * 60;
const NORFOLK_ISLAND_WINTER_UTC_OFFSET_SECONDS: i64 = 11 * 60 * 60;

fn make_date_time_with_offset(
    year: i32,
    month: u32,
    day: u32,
    hour: u32,
    min: u32,
    offset_seconds: i64,
) -> NaiveDateTime {
    NaiveDateTime::new(
        NaiveDate::from_ymd_opt(year, month, day).unwrap(),
        NaiveTime::from_hms_opt(hour, min, 0).unwrap(),
    ) - Duration::seconds(offset_seconds)
}

#[test]
fn load_config() {
    let config: Result<DeviceConfig, _> = toml::from_str(
        r#"
[device]
id = 1
group = "test-group"
name = "test-name"
server = "test-url"

[thermal-recorder]
use-sunrise-sunset = false
max-secs = 300
min-disk-space-mb = 200
min-secs = 5
output-dir = "/var/spool/cptv"
preview-secs = 1

[location]
accuracy = 0.0
altitude = 103.0
latitude = -46.60101
longitude = 172.71303
timestamp = 2023-11-02T08:24:21+13:00
updated = 2023-11-02T08:24:21+13:00

[thermal-throttler]
activate = true

[windows]
start-recording = "12:00"
stop-recording = "11:00"
"#,
    );
    assert!(config.is_ok());
}
