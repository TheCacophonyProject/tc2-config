use crate::tests::{make_time_with_offset, NZ_SUMMER_UTC_OFFSET_SECONDS};
use crate::DeviceConfig;
use chrono::Duration;

#[test]
fn test_start_less_than_end() {
    let config: Result<DeviceConfig, _> = toml::from_str(
        r#"
[windows]
start-recording = "09:10"
stop-recording = "17:30"
"#,
    );
    assert!(config.is_ok());
    if let Ok(config) = config {
        let now = make_time_with_offset(9, 9, NZ_SUMMER_UTC_OFFSET_SECONDS);
        let (start, _) = config.next_recording_window(&now);
        assert!(
            !config.time_is_in_recording_window(&now),
            "Not in the active window"
        );

        assert_eq!(
            start - now,
            Duration::minutes(1),
            "It should be 1 minute until the start of the next window"
        );

        let now = make_time_with_offset(9, 10, NZ_SUMMER_UTC_OFFSET_SECONDS);
        let (start, _) = config.next_recording_window(&now);
        assert!(config.time_is_in_recording_window(&now), "In active window");
        assert_eq!(
            start - now,
            Duration::minutes(0),
            "It should be 0 minutes until the start of the next window"
        );

        let now = make_time_with_offset(12, 0, NZ_SUMMER_UTC_OFFSET_SECONDS);
        let (_, end) = config.next_recording_window(&now);
        assert!(config.time_is_in_recording_window(&now), "In active window");
        assert_eq!(
            end - now,
            Duration::minutes(5 * 60 + 30),
            "It should be 330 minutes until the end of the current window"
        );

        let now = make_time_with_offset(17, 29, NZ_SUMMER_UTC_OFFSET_SECONDS);
        let (_, end) = config.next_recording_window(&now);
        assert!(config.time_is_in_recording_window(&now), "In active window");
        assert_eq!(
            end - now,
            Duration::minutes(1),
            "It should be 1 minutes until the end of the current window"
        );

        let now = make_time_with_offset(17, 30, NZ_SUMMER_UTC_OFFSET_SECONDS);
        let (start, _) = config.next_recording_window(&now);
        assert!(
            !config.time_is_in_recording_window(&now),
            "Outside active window"
        );
        assert_eq!(
            start - now,
            Duration::minutes(940),
            "It should be 940 minutes until the start of the next window"
        );
    }
}

#[test]
fn test_start_greater_than_end() {
    let config: Result<DeviceConfig, _> = toml::from_str(
        r#"
[windows]
start-recording = "22:10"
stop-recording = "9:50"
"#,
    );
    assert!(config.is_ok());
    if let Ok(config) = config {
        let now = make_time_with_offset(22, 9, NZ_SUMMER_UTC_OFFSET_SECONDS);
        let (start, _) = config.next_recording_window(&now);
        assert!(
            !config.time_is_in_recording_window(&now),
            "Not in the active window"
        );

        assert_eq!(
            start - now,
            Duration::minutes(1),
            "It should be 1 minute until the start of the next window"
        );

        let now = make_time_with_offset(22, 10, NZ_SUMMER_UTC_OFFSET_SECONDS);
        let (start, _) = config.next_recording_window(&now);
        assert!(config.time_is_in_recording_window(&now), "In active window");
        assert_eq!(
            start - now,
            Duration::minutes(0),
            "It should be 0 minutes until the start of the next window"
        );

        let now = make_time_with_offset(23, 59, NZ_SUMMER_UTC_OFFSET_SECONDS);
        let (_, end) = config.next_recording_window(&now);
        assert!(config.time_is_in_recording_window(&now), "In active window");
        assert_eq!(
            end - now,
            Duration::minutes(9 * 60 + 51),
            "It should be 591 minutes until the end of the current window"
        );

        let now = make_time_with_offset(0, 0, NZ_SUMMER_UTC_OFFSET_SECONDS);
        let (_, end) = config.next_recording_window(&now);
        //println!("Now {}, start {}, end {}", now, start, end);
        assert!(config.time_is_in_recording_window(&now), "In active window");
        assert_eq!(
            end - now,
            Duration::minutes(9 * 60 + 50),
            "It should be 590 minutes until the end of the current window"
        );

        let now = make_time_with_offset(0, 1, NZ_SUMMER_UTC_OFFSET_SECONDS);
        let (_, end) = config.next_recording_window(&now);
        assert!(config.time_is_in_recording_window(&now), "In active window");
        assert_eq!(
            end - now,
            Duration::minutes(589),
            "It should be 589 minutes until the end of the current window"
        );

        let now = make_time_with_offset(2, 0, NZ_SUMMER_UTC_OFFSET_SECONDS);
        assert!(config.time_is_in_recording_window(&now), "In active window");

        let now = make_time_with_offset(9, 49, NZ_SUMMER_UTC_OFFSET_SECONDS);
        let (_, end) = config.next_recording_window(&now);
        assert!(config.time_is_in_recording_window(&now), "In active window");
        assert_eq!(
            end - now,
            Duration::minutes(1),
            "It should be 1 minutes until the end of the current window"
        );

        let now = make_time_with_offset(9, 50, NZ_SUMMER_UTC_OFFSET_SECONDS);
        let (_, end) = config.next_recording_window(&now);
        assert!(
            config.time_is_in_recording_window(&now),
            "Inside active window"
        );
        assert_eq!(
            end - now,
            Duration::minutes(0),
            "It should be 0 minutes until the end of the current window"
        );

        let now = make_time_with_offset(9, 51, NZ_SUMMER_UTC_OFFSET_SECONDS);
        let (start, _) = config.next_recording_window(&now);
        assert!(
            !config.time_is_in_recording_window(&now),
            "Outside active window"
        );
        assert_eq!(
            start - now,
            Duration::minutes(739),
            "It should be 739 minutes until the start of the next window"
        );
    }
}

#[test]
fn test_morning_to_morning() {
    let config: DeviceConfig = toml::from_str(
        r#"
[windows]
start-recording = "11:00"
stop-recording = "10:00"
"#,
    )
    .unwrap();
    let now = make_time_with_offset(9, 59, NZ_SUMMER_UTC_OFFSET_SECONDS);
    let (_, end) = config.next_recording_window(&now);
    assert!(
        config.time_is_in_recording_window(&now),
        "Inside active window"
    );
    assert_eq!(
        end - now,
        Duration::minutes(1),
        "It should be 1 minute until the end of the current window"
    );

    let now = make_time_with_offset(10, 0, NZ_SUMMER_UTC_OFFSET_SECONDS);
    let (start, _) = config.next_recording_window(&now);
    assert!(
        !config.time_is_in_recording_window(&now),
        "Outside active window"
    );
    assert_eq!(
        start - now,
        Duration::minutes(60),
        "It should be 60 minutes until the start of the next window"
    );

    let now = make_time_with_offset(10, 59, NZ_SUMMER_UTC_OFFSET_SECONDS);
    let (start, _) = config.next_recording_window(&now);
    assert!(
        !config.time_is_in_recording_window(&now),
        "Outside active window"
    );
    assert_eq!(
        start - now,
        Duration::minutes(1),
        "It should be 1 minute until the start of the next window"
    );

    let now = make_time_with_offset(11, 0, NZ_SUMMER_UTC_OFFSET_SECONDS);
    let (_, end) = config.next_recording_window(&now);
    assert!(
        config.time_is_in_recording_window(&now),
        "Inside active window"
    );
    assert_eq!(
        end - now,
        Duration::minutes(23 * 60),
        "It should be 1380 minutes until the end of the current window"
    );

    let now = make_time_with_offset(18, 0, NZ_SUMMER_UTC_OFFSET_SECONDS);
    let (_, end) = config.next_recording_window(&now);
    assert!(
        config.time_is_in_recording_window(&now),
        "Inside active window"
    );
    assert_eq!(
        end - now,
        Duration::minutes(16 * 60),
        "It should be 960 minutes until the end of the current window"
    );
}

#[test]
fn test_same_start_end() {
    let config: DeviceConfig = toml::from_str(
        r#"
[windows]
start-recording = "15:04"
stop-recording = "15:04"
"#,
    )
    .unwrap();
    assert!(config.time_is_in_recording_window(&chrono::Utc::now().naive_utc()));
    assert!(config.is_continuous_recorder());
}
