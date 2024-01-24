use crate::sun_times::sun_times;
use crate::tests::{
    make_date_time_with_offset, CAYMAN_ISLANDS_UTC_OFFSET_SECONDS,
    NORFOLK_ISLAND_SUMMER_UTC_OFFSET_SECONDS, NORFOLK_ISLAND_WINTER_UTC_OFFSET_SECONDS,
    NZ_SUMMER_UTC_OFFSET_SECONDS, NZ_WINTER_UTC_OFFSET_SECONDS, UK_SUMMER_UTC_OFFSET_SECONDS,
    UK_WINTER_UTC_OFFSET_SECONDS,
};
use crate::DeviceConfig;
use chrono::{Duration, NaiveDate};

#[test]
fn test_sunrise_sunset_nz() {
    let test_latitude: f64 = -41.0;
    let test_longitude: f64 = 175.0;

    for month in 10..=12 {
        test_location_in_month(
            test_latitude,
            test_longitude,
            month,
            NZ_SUMMER_UTC_OFFSET_SECONDS,
        );
    }
    for month in 1..=3 {
        test_location_in_month(
            test_latitude,
            test_longitude,
            month,
            NZ_SUMMER_UTC_OFFSET_SECONDS,
        );
    }

    for month in 5..8 {
        test_location_in_month(
            test_latitude,
            test_longitude,
            month,
            NZ_WINTER_UTC_OFFSET_SECONDS,
        );
    }
}

#[test]
fn test_sunrise_sunset_caymans() {
    let test_latitude: f64 = 19.5;
    let test_longitude: f64 = -81.23;
    for month in 1..=12 {
        test_location_in_month(
            test_latitude,
            test_longitude,
            month,
            CAYMAN_ISLANDS_UTC_OFFSET_SECONDS,
        );
    }
}

#[test]
fn test_sunrise_sunset_norfolk() {
    let test_latitude: f64 = -29.0;
    let test_longitude: f64 = 167.8;
    for month in 11..=12 {
        test_location_in_month(
            test_latitude,
            test_longitude,
            month,
            NORFOLK_ISLAND_SUMMER_UTC_OFFSET_SECONDS,
        );
    }
    for month in 1..=3 {
        test_location_in_month(
            test_latitude,
            test_longitude,
            month,
            NORFOLK_ISLAND_SUMMER_UTC_OFFSET_SECONDS,
        );
    }
    for month in 5..=9 {
        test_location_in_month(
            test_latitude,
            test_longitude,
            month,
            NORFOLK_ISLAND_WINTER_UTC_OFFSET_SECONDS,
        );
    }
}

#[test]
fn test_sunrise_sunset_uk() {
    let test_latitude: f64 = 51.5;
    let test_longitude: f64 = 0.12;

    for month in 11..=12 {
        test_location_in_month(
            test_latitude,
            test_longitude,
            month,
            UK_WINTER_UTC_OFFSET_SECONDS,
        );
    }
    for month in 1..=3 {
        test_location_in_month(
            test_latitude,
            test_longitude,
            month,
            UK_WINTER_UTC_OFFSET_SECONDS,
        );
    }

    for month in 4..=10 {
        test_location_in_month(
            test_latitude,
            test_longitude,
            month,
            UK_SUMMER_UTC_OFFSET_SECONDS,
        );
    }
}

fn test_location_in_month(
    test_latitude: f64,
    test_longitude: f64,
    month: u32,
    utc_offset_seconds: i64,
) {
    let config: Result<DeviceConfig, _> = toml::from_str(&format!(
        r#"
[location]
accuracy = 0.0
altitude = 0.0
latitude = {}
longitude = {}

[windows]
start-recording = "-1h"
stop-recording = "2h"
"#,
        test_latitude, test_longitude
    ));
    assert!(config.is_ok());
    let config = config.unwrap();
    assert!(config
        .recording_window
        .start_recording
        .relative_time_seconds
        .is_some());
    assert_eq!(
        config
            .recording_window
            .start_recording
            .relative_time_seconds
            .unwrap(),
        -(60 * 60),
        "End time should be -3600 (1m) seconds before sunset"
    );

    assert!(config
        .recording_window
        .stop_recording
        .relative_time_seconds
        .is_some());
    assert_eq!(
        config
            .recording_window
            .stop_recording
            .relative_time_seconds
            .unwrap(),
        2 * (60 * 60),
        "End time should be 7200 (2h) seconds after sunrise"
    );

    assert!(config.location.as_ref().unwrap().latitude.unwrap() as f64 - test_latitude < 0.01);
    assert!(config.location.as_ref().unwrap().longitude.unwrap() as f64 - test_longitude < 0.01);

    let not_active_date = make_date_time_with_offset(2000, month, 2, 12, 0, utc_offset_seconds);
    let today_sun_times = sun_times(
        NaiveDate::from_ymd_opt(2000, month, 2).unwrap(),
        test_latitude,
        test_longitude,
        0.0,
    );
    assert!(today_sun_times.is_some());
    let (_, today_sunset) = today_sun_times.unwrap();
    let tomorrow_sun_times = sun_times(
        NaiveDate::from_ymd_opt(2000, month, 3).unwrap(),
        test_latitude,
        test_longitude,
        0.0,
    );
    assert!(tomorrow_sun_times.is_some());
    let (tomorrow_sunrise, _) = tomorrow_sun_times.unwrap();
    let (next_window_start, next_window_end) = config.next_recording_window(&not_active_date);

    assert_eq!(
        next_window_start,
        (today_sunset - Duration::hours(1)).naive_utc(),
        "Now is {}(UTC). Next window start ({}) should be 1hr before todays sunset ({})",
        not_active_date,
        next_window_start,
        today_sunset
    );

    assert_eq!(
        next_window_end,
        (tomorrow_sunrise + Duration::hours(2)).naive_utc(),
        "Now is {}(UTC). Next window end ({}) should be 2hrs after tomorrows sunrise ({})",
        not_active_date,
        next_window_end,
        tomorrow_sunrise
    );

    assert!(
        !config.time_is_in_recording_window(&not_active_date),
        "Time should be outside recording window"
    );

    let active_now_date = make_date_time_with_offset(2000, month, 2, 21, 1, utc_offset_seconds);
    let (next_window_start, next_window_end) = config.next_recording_window(&active_now_date);
    assert_eq!(
        (today_sunset - Duration::hours(1)).naive_utc(),
        next_window_start,
        "Now is {}(UTC). Next window start ({}) should be 1hr before todays sunset ({})",
        active_now_date,
        next_window_start,
        today_sunset
    );
    assert_eq!(
        (tomorrow_sunrise + Duration::hours(2)).naive_utc(),
        next_window_end,
        "Now is {}(UTC). Next window end ({}) should be 2hrs after tomorrows sunrise ({})",
        active_now_date,
        next_window_end,
        tomorrow_sunrise
    );
    assert!(
        config.time_is_in_recording_window(&active_now_date),
        "Time should be inside recording window: Now {}, {}, {}",
        active_now_date,
        next_window_start,
        next_window_end
    );
}
