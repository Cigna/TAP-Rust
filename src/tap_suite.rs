use crate::tap_test::TapTest;

use std::io::Write;

/// Represents a collection of TAP tests (`TapTest`) which can be rendered into a (text) TAP stream. This orchestrates that rendering.
#[derive(Debug, Clone)]
pub struct TapSuite {
    /// The name of the suite. If this is a blank string, that's fine but it's considered a party foul.
    pub name: String,
    /// The collection of `TapTest` objects included in this test group, to be rendered into a TAP stream.
    pub tests: Vec<TapTest>,
}

impl TapSuite {
    /// Produce and arrange all text lines, in order, included in this TAP stream. This includes the leading plan line which is calculated based on the number of tests.
    pub fn lines(&self) -> Vec<String> {
        // Make plan line
        let first_line = format!("1..{}", self.tests.len());
        let mut all_lines = vec![first_line];

        for (i, test) in self.tests.iter().enumerate() {
            let index = i as i64; // by default i is a usize.
            let tap = test.tap(index + 1); // TAP tests can't start with zero
            all_lines.extend(tap.iter().cloned());
        }

        all_lines
    }

    /// Emit TAP stream to the provided sink, which must be `Write`.
    pub fn print<T: Write>(&self, mut sink: T) -> Result<String, String> {
        let output = self.lines().join("");
        match sink.write_all(output.as_bytes()) {
            Ok(_) => Result::Ok(output),
            Err(reason) => Result::Err(reason.to_string()),
        }
    }
}

impl PartialEq for TapSuite {
    fn eq(&self, other: &TapSuite) -> bool {
        self.name == other.name && self.tests == other.tests
    }
}

#[cfg(test)]
mod tests {
    use super::TapSuite;
    use crate::tap_test_builder::TapTestBuilder;

    #[test]
    fn test_lines() {
        let passing_test = TapTestBuilder::new()
            .name("Panda Bamboo")
            .passed(true)
            .finalize();

        let failing_test = TapTestBuilder::new()
            .name("Curry Noodle")
            .passed(false)
            .diagnostics(&vec!["Tree", "Flower"])
            .finalize();

        let tap_suite = TapSuite {
            name: "Example TAP Suite".to_string(),
            tests: vec![passing_test, failing_test],
        };

        let expected = vec![
            "1..2",
            "ok 1 Panda Bamboo",
            "not ok 2 Curry Noodle",
            "# Tree",
            "# Flower",
        ];
        let actual = tap_suite.lines();

        assert_eq!(expected, actual);
    }
}
