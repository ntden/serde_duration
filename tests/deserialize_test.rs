use serde::{Deserialize, Serialize};
use serde_json::{self, json};
use std::time::Duration;

#[derive(Debug, Serialize, Deserialize)]
struct TestStruct {
    #[serde(with = "serde_duration")]
    duration: Duration,
}

#[test]
fn test_deserialize() {
    let json_str = json!({"duration": "1234s"}).to_string();
    let deserialized: TestStruct = serde_json::from_str(&json_str)
        .unwrap_or_else(|e| panic!("Failed to deserialize JSON: {}", e));
    assert_eq!(deserialized.duration, Duration::from_secs(1234));
}
