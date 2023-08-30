//! `TapTest` -- The core, representing an individual TAP test.

#[cfg(feature = "alloc")]
use alloc::{
    string::{String, ToString},
    vec,
    vec::Vec,
};
use core::fmt::Write;
use std::fmt;

use crate::{NOT_OK_SYMBOL, OK_SYMBOL};

/// A test, a collection of which (a `TapSuite`) will be rendered into a TAP text stream. A `TapTest` knows how to render itself.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TapTest {
    /// The name of the test, will be the primary text on a TAP test line
    pub name: String,
    /// Did this test pass?
    pub passed: bool,
    /// If this test merits additional comments (diagnostics, in TAP parlance), they will be rendered in the TAP stream beginning with a # mark.
    pub diagnostics: Vec<String>,
}

impl TapTest {
    /// Based on the test passing status, yield either "ok" or "not ok".
    pub fn ok_string(&self) -> String {
        if self.passed {
            OK_SYMBOL
        } else {
            NOT_OK_SYMBOL
        }
        .to_string()
    }

    /// Produce a properly-formatted TAP line. This excludes diagnostics.
    pub fn status_line(&self, test_number: i64) -> String {
        let ok_string = self.ok_string();
        let test_number_string = test_number.to_string();
        let mut buf =
            String::with_capacity(ok_string.len() + test_number_string.len() + self.name.len());
        write!(&mut buf, "{} {} {}", ok_string, test_number, self.name).unwrap();
        buf
    }

    /// Produce all lines (inclusive of diagnostics) representing this test. This is the money, right here.
    pub fn tap(&self, test_number: i64) -> Vec<String> {
        // Build the first line
        let mut lines = vec![self.status_line(test_number)];
        // If there are diagnostics lines, format them.
        let formatted_diagnostics = self
            .diagnostics
            .iter()
            .map(|comment| self.format_diagnostics(comment))
            .collect::<Vec<String>>();

        lines.extend(formatted_diagnostics.iter().cloned());

        lines
    }

    /// Diagnostics should begin with a # mark
    pub fn format_diagnostics(&self, line: &str) -> String {
        let mut buf = String::with_capacity(line.len() + 2);
        write!(&mut buf, "# {}", line).unwrap();
        buf
    }
}

impl From<TapTest> for String {
    fn from(tap_test: TapTest) -> Self {
        let mut buf = String::new();
        write!(
            &mut buf,
            "TapTest(name: {}, passed: {}, diagnostics: {:?})",
            tap_test.name, tap_test.passed, tap_test.diagnostics
        )
        .unwrap();
        buf
    }
}

impl From<&TapTest> for String {
    fn from(tap_test: &TapTest) -> Self {
        let mut buf = String::new();
        write!(
            &mut buf,
            "TapTest(name: {}, passed: {}, diagnostics: {:?})",
            tap_test.name, tap_test.passed, tap_test.diagnostics
        )
        .unwrap();
        buf
    }
}

impl fmt::Display for TapTest {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", String::from(self))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tap_test_status_string() {
        let tap_test_passing = TapTest {
            name: "Panda".to_string(),
            passed: true,
            diagnostics: vec!["Doing fine".to_string()],
        };

        let expected_passing = OK_SYMBOL;
        let actual_passing = tap_test_passing.ok_string();

        assert_eq!(expected_passing, actual_passing);

        let tap_test_failing = TapTest {
            name: "Panda".to_string(),
            passed: false,
            diagnostics: vec!["Doing fine".to_string()],
        };

        let expected_failing = NOT_OK_SYMBOL;
        let actual_failing = tap_test_failing.ok_string();

        assert_eq!(expected_failing, actual_failing);
    }

    #[test]
    fn test_format_first_line() {
        let tap_test_passing = TapTest {
            name: "Panda".to_string(),
            passed: true,
            diagnostics: vec!["Doing fine".to_string()],
        };

        let expected_passing = "ok 42 Panda";
        let actual_passing = tap_test_passing.status_line(42);

        assert_eq!(expected_passing, actual_passing);

        let tap_test_failing = TapTest {
            name: "Panda".to_string(),
            passed: false,
            diagnostics: vec!["Doing fine".to_string()],
        };

        let expected_failing = "not ok 42 Panda";
        let actual_failing = tap_test_failing.status_line(42);

        assert_eq!(expected_failing, actual_failing);
    }

    #[test]
    fn test_tap_lines() {
        let tap_test_passing = TapTest {
            name: "Panda".to_string(),
            passed: true,
            diagnostics: vec!["Doing fine".to_string()],
        };

        let expected_passing = vec!["ok 42 Panda", "# Doing fine"];
        let actual_passing = tap_test_passing.tap(42);

        assert_eq!(expected_passing, actual_passing);
    }

    #[test]
    fn test_format_diagnostics() {
        let tap_test_passing = TapTest {
            name: "Panda".to_string(),
            passed: true,
            diagnostics: vec!["Doing fine".to_string()],
        };

        let expected_passing = "# Doing fine";
        let actual_passing = tap_test_passing.format_diagnostics(&tap_test_passing.diagnostics[0]);

        assert_eq!(expected_passing, actual_passing);
    }
}
