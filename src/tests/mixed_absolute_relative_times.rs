use crate::sun_times::sun_times;
use crate::tests::{make_date_time_with_offset, NZ_SUMMER_UTC_OFFSET_SECONDS};
use crate::DeviceConfig;
use chrono::{Duration, NaiveDate};

#[test]
fn test_mixed_relative_and_abs() {
    let test_latitude: f64 = -41.0;
    let test_longitude: f64 = 175.0;

    let tomorrow_sun_times = sun_times(
        NaiveDate::from_ymd_opt(2000, 1, 2).unwrap(),
        test_latitude,
        test_longitude,
        0.0,
    );
    assert!(tomorrow_sun_times.is_some());
    let (tomorrow_sunrise, _) = tomorrow_sun_times.unwrap();
    let today_sun_times = sun_times(
        NaiveDate::from_ymd_opt(2000, 1, 1).unwrap(),
        test_latitude,
        test_longitude,
        0.0,
    );
    assert!(today_sun_times.is_some());
    let (_, today_sunset) = today_sun_times.unwrap();

    let now = make_date_time_with_offset(2000, 1, 2, 1, 1, NZ_SUMMER_UTC_OFFSET_SECONDS);

    let config: Result<DeviceConfig, _> = toml::from_str(&format!(
        r#"
[location]
accuracy = 0.0
altitude = 0.0
latitude = {}
longitude = {}

[windows]
start-recording = "13:00"
stop-recording = "1h"
"#,
        test_latitude, test_longitude
    ));
    assert!(config.is_ok());
    let config = config.unwrap();
    let (_, end) = config.next_recording_window(&now);
    assert_eq!(
        (tomorrow_sunrise + Duration::hours(1)).naive_utc(),
        end,
        "Next window should end 1hr past tomorrows sunrise"
    );

    let config: Result<DeviceConfig, _> = toml::from_str(&format!(
        r#"
[location]
accuracy = 0.0
altitude = 0.0
latitude = {}
longitude = {}

[windows]
start-recording = "-1h"
stop-recording = "13:00"
"#,
        test_latitude, test_longitude
    ));
    assert!(config.is_ok());
    let config = config.unwrap();
    let (start, _) = config.next_recording_window(&now);
    println!("Now {}, start {}", now, start);
    assert_eq!(
        start,
        (today_sunset - Duration::minutes(60)).naive_utc(),
        "Next window should start 1hr before todays sunset"
    );
}
