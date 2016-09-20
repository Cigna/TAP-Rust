/// A test, a collection of which (a `TapSuite`) will be rendered into a TAP text stream. A `TapTest` knows how to render itself.
#[derive(Debug)]
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
        let result = if self.passed {
            "ok"
        } else {
            "not ok"
        };

        result.to_string()
    }

    /// Produce a properly-formatted TAP line. This excludes diagnostics.
    pub fn status_line(&self, test_number: i64) -> String {
        format!("{} {} {}", self.ok_string(), test_number, self.name)
    }

    /// Produce all lines (inclusive of diagnostics) representing this test. This is the money, right here.
    pub fn tap(&self, test_number: i64) -> Vec<String> {
        // Build the first line
        let mut lines = vec![self.status_line(test_number)];
        // If there are diagnostics lines, format them.
        let formatted_diagnostics = self.diagnostics
            .iter()
            .map(|comment| self.format_diagnostics(comment))
            .collect::<Vec<String>>();

        lines.extend(formatted_diagnostics.iter().cloned());

        lines
    }

    /// Diagnostics should begin with a # mark
    pub fn format_diagnostics(&self, line: &str) -> String {
        format!("# {}", line)
    }
}

impl PartialEq for TapTest {
    fn eq(&self, other: &TapTest) -> bool {
        self.name == other.name
            && self.passed == other.passed
            && self.diagnostics == other.diagnostics
    }
}


#[cfg(test)]
mod tests {
    use super::TapTest;

    #[test]
    fn test_tap_test_status_string() {
        let tap_test_passing = TapTest {
            name: "Panda".to_string(),
            passed: true,
            diagnostics: vec!["Doing fine".to_string()],
        };

        let expected_passing = "ok";
        let actual_passing = tap_test_passing.ok_string();

        assert_eq!(expected_passing, actual_passing);

        let tap_test_failing = TapTest {
            name: "Panda".to_string(),
            passed: false,
            diagnostics: vec!["Doing fine".to_string()],
        };

        let expected_failing = "not ok";
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
