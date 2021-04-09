//! This crate provides utility functions for serializing and deserializing
//! `Duration` objects in a custom format. The format uses the letters
//! "s", "m", and "h" to denote seconds, minutes, and hours, respectively.
//! 
//! For example, "10s" represents a duration of 10 seconds,
//! "5m" represents a duration of 5 minutes, and "3h" represents a duration of
//! 3 hours. The functions provided by this crate allow you to easily convert
//! between `Duration` objects and these custom strings.
//!
//! # Examples
//!
//! ```
//! use std::time::Duration;
//! use serde::{Serialize, Deserialize};
//! 
//! use serde_duration;
//!
//! #[derive(Serialize, Deserialize)]
//! struct TestStruct {
//!     #[serde(with = "serde_duration")]
//!     duration: Duration,
//! }
//!
//! let config_str = r#"{"timeout": "30s"}"#;
//! let config: MyConfig = serde_json::from_str(config_str).unwrap();
//! assert_eq!(config.timeout, Duration::from_secs(30));
//! ```

use serde::{de, Deserialize, Deserializer, Serializer};
use std::time::Duration;

#[derive(Debug)]
struct InvalidDurationError;

impl std::error::Error for InvalidDurationError {}

impl std::fmt::Display for InvalidDurationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Invalid duration format")
    }
}

/// Serializes a duration to a string using the format "Xs", "Xm", or "Xh",
/// where X is the duration in seconds, minutes, or hours, respectively.
///
/// # Arguments
///
/// * `duration` - The duration to serialize
/// * `serializer` - The serde serializer
///
/// # Returns
///
/// A result containing the serialized string if serialization was successful,
/// or an error if serialization failed.
pub fn serialize<S>(duration: &Duration, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str(&duration_to_str(duration))
}

/// Deserializes a duration from a string using the format "Xs", "Xm", or "Xh",
/// where X is the duration in seconds, minutes, or hours, respectively.
///
/// # Arguments
///
/// * `deserializer` - The serde deserializer
///
/// # Returns
///
/// A result containing the deserialized duration if deserialization was successful,
/// or an error if deserialization failed.
pub fn deserialize<'de, D>(deserializer: D) -> Result<Duration, D::Error>
where
    D: Deserializer<'de>,
{
    // Deserialize the input string using the given deserializer
    let s = match String::deserialize(deserializer) {
        Ok(s) => s,
        Err(e) => return Err(e),
    };
    
    // Attempt to convert the string to a duration
    match str_to_duration(&s) {
        Ok(Some(duration)) => Ok(duration),
        Ok(None) => Err(de::Error::custom("invalid duration format")),
        Err(e) => Err(de::Error::custom(e)),
    }
}

fn str_to_duration(s: &str) -> Result<Option<Duration>, InvalidDurationError> {
    let multiplier = match s.chars().last() {
        Some('s') => Duration::from_secs(1),
        Some('m') => Duration::from_secs(60),
        Some('h') => Duration::from_secs(3600),
        _ => return Ok(None),
    };
    let value = s[..s.len() - 1]
        .parse::<u64>()
        .map_err(|_| InvalidDurationError)?;
    Ok(Some(Duration::from_secs(multiplier.as_secs() * value)))
}

fn duration_to_str(duration: &Duration) -> String {
    let seconds = duration.as_secs();
    if seconds >= 3600 {
        format!("{}h", seconds / 3600)
    } else if seconds >= 60 {
        format!("{}m", seconds / 60)
    } else {
        format!("{}s", seconds)
    }
}
