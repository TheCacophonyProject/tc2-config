use crate::DeviceConfig;

#[test]
fn test_time_parsing() {
    let config: Result<DeviceConfig, _> = toml::from_str(
        r#"
[windows]
start-recording = "20:10"
stop-recording = "08:00"
"#,
    );
    assert!(config.is_ok());
    let config = config.unwrap();
    assert!(
        config
            .recording_window
            .start_recording
            .relative_time_seconds
            .is_none(),
        "Start should not be relative"
    );
    assert_eq!(
        config
            .recording_window
            .start_recording
            .absolute_time
            .as_ref()
            .unwrap()
            .hour,
        20
    );
    assert_eq!(
        config
            .recording_window
            .start_recording
            .absolute_time
            .as_ref()
            .unwrap()
            .min,
        10
    );
    assert!(
        config
            .recording_window
            .stop_recording
            .relative_time_seconds
            .is_none(),
        "End should not be relative"
    );
    assert_eq!(
        config
            .recording_window
            .stop_recording
            .absolute_time
            .as_ref()
            .unwrap()
            .hour,
        8
    );
    assert_eq!(
        config
            .recording_window
            .stop_recording
            .absolute_time
            .as_ref()
            .unwrap()
            .min,
        0
    );

    let config: Result<DeviceConfig, _> = toml::from_str(
        r#"
[windows]
start-recording = "-1h20m"
stop-recording = "10:31"
"#,
    );
    assert!(config.is_ok());
    let config = config.unwrap();
    assert!(
        config
            .recording_window
            .start_recording
            .relative_time_seconds
            .is_some(),
        "Start time should be relative"
    );

    assert_eq!(
        config
            .recording_window
            .start_recording
            .relative_time_seconds
            .unwrap(),
        -(80 * 60),
        "Start time should be -4800 (1h20m) seconds before sunrise"
    );

    assert!(
        config
            .recording_window
            .stop_recording
            .absolute_time
            .is_some(),
        "End time should be absolute"
    );

    assert_eq!(
        config
            .recording_window
            .stop_recording
            .absolute_time
            .as_ref()
            .unwrap()
            .hour,
        10
    );
    assert_eq!(
        config
            .recording_window
            .stop_recording
            .absolute_time
            .as_ref()
            .unwrap()
            .min,
        31
    );

    let config: Result<DeviceConfig, _> = toml::from_str(
        r#"
[windows]
start-recording = "21:59"
stop-recording = "3h"
"#,
    );
    assert!(config.is_ok());
    let config = config.unwrap();
    assert!(
        config
            .recording_window
            .start_recording
            .absolute_time
            .is_some(),
        "Start time should be absolute"
    );
    assert_eq!(
        config
            .recording_window
            .start_recording
            .absolute_time
            .as_ref()
            .unwrap()
            .hour,
        21
    );
    assert_eq!(
        config
            .recording_window
            .start_recording
            .absolute_time
            .as_ref()
            .unwrap()
            .min,
        59
    );
    assert!(
        config
            .recording_window
            .stop_recording
            .relative_time_seconds
            .is_some(),
        "End time should be relative"
    );
    assert_eq!(
        config
            .recording_window
            .stop_recording
            .relative_time_seconds
            .unwrap(),
        3 * 60 * 60,
        "End time should be 10800 (3h) seconds after sunrise"
    );

    let config: Result<DeviceConfig, _> = toml::from_str(
        r#"
[windows]
start-recording = "30m"
stop-recording = "-1h45m"
"#,
    );
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
        30 * 60,
        "End time should be 1800 (30m) seconds after sunset"
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
        -((60 * 60) + (45 * 60)),
        "End time should be -6300 (-1h45m) seconds before sunrise"
    );

    let config: Result<DeviceConfig, _> = toml::from_str(
        r#"
[windows]
start-recording = "abc"
stop-recording = "1:30"
"#,
    );
    //println!("Config {:?}", config.unwrap());
    assert!(config.is_err());
    let config: Result<DeviceConfig, _> = toml::from_str(
        r#"
[windows]
start-recording = "1:30"
stop-recording = "abc"
"#,
    );
    assert!(config.is_err());

    let config: Result<DeviceConfig, _> = toml::from_str(
        r#"
[windows]
start-recording = "h:30"
stop-recording = "1:30"
"#,
    );
    assert!(config.is_err());

    let config: Result<DeviceConfig, _> = toml::from_str(
        r#"
[windows]
start-recording = ":30"
stop-recording = "1:30"
"#,
    );
    assert!(config.is_err());

    let config: Result<DeviceConfig, _> = toml::from_str(
        r#"
[windows]
start-recording = "-1a"
stop-recording = "1:30"
"#,
    );
    assert!(config.is_err());
}
