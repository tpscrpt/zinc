//!
//! The Zinc tester metadata case.
//!

use serde::Deserialize;
use serde_json::Value as JsonValue;

///
/// The test file case.
///
#[derive(Debug, Deserialize, PartialEq)]
pub struct Case {
    /// The case name.
    pub case: String,
    /// The entry or method which must be run for the case.
    pub method: Option<String>,
    /// The entry or method input data.
    pub input: JsonValue,
    /// The entry or method expected output data.
    pub output: JsonValue,
    /// If the test case must return an error to be successful.
    #[serde(default)]
    pub should_panic: bool,
    /// If the test case must be ignored.
    #[serde(default)]
    pub ignore: bool,
}
