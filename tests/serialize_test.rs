use serde::{Deserialize, Serialize};
use serde_json;
use std::time::Duration;

#[derive(Debug, Serialize, Deserialize)]
struct TestStruct {
    #[serde(with = "serde_duration")]
    duration: Duration,
}

#[test]
fn test_serialize() {
    let tests = vec![
        (Duration::from_secs(1200), "{\"duration\":\"20m\"}"),
        (Duration::from_secs(5), "{\"duration\":\"5s\"}"),
        (Duration::from_secs(1200 * 60), "{\"duration\":\"20h\"}"),
    ];

    for (duration, expected) in tests {
        let test_struct = TestStruct { duration };
        let serialized = serde_json::to_string(&test_struct).unwrap();
        assert_eq!(serialized, expected);
    }
}
