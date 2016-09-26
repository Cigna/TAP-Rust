use tap_suite::TapSuite;
use tap_test::TapTest;

/// Coordinator for constructing `TapSuite` objects using the builder pattern.
///
/// # Examples
///
/// ```
/// use tap::tap_suite_builder::TapSuiteBuilder;
/// use tap::tap_test_builder::TapTestBuilder;
///
/// // Make a Vec<TapTest> so we have something
/// let tests = vec![TapTestBuilder::new()
///                  .name("Example TAP test")
///                  .passed(true)
///                  .finalize()];
///
/// let tap_suite_from_builder = TapSuiteBuilder::new()
///     .name("Example TAP test suite")
///     .tests(tests)
///     .finalize();
///
/// ```
#[derive(Debug, Clone)]
pub struct TapSuiteBuilder {
    /// Name of test suite
    pub name: Option<String>,
    /// Vector of type `Vec<TapTest>` which holds the actual tests
    pub tests: Option<Vec<TapTest>>,
}

impl TapSuiteBuilder {
    /// Produce a new builder object
    pub fn new() -> TapSuiteBuilder {
        TapSuiteBuilder { name: None, tests: None }
    }
    /// Set the name
    pub fn name<S: Into<String>>(&mut self, s: S) -> &mut TapSuiteBuilder {
        self.name = Some(s.into());
        self
    }
    /// Set the tests
    pub fn tests(&mut self, test_vec: Vec<TapTest>) -> &mut TapSuiteBuilder {
        self.tests = Some(test_vec);
        self
    }
    /// Produce the configured `TapSuite` object. Name defaults to a blank `String` and the tests default to an empty `Vec`.
    pub fn finalize(&mut self) -> TapSuite {
        TapSuite {
            name: self.name.take().unwrap_or("".to_string()),
            tests: self.tests.take().unwrap_or(Vec::new()),
        }
    }
}

#[cfg(test)]
mod test {
    use super::TapSuiteBuilder;
    use tap_suite::TapSuite;
    use tap_test_builder::TapTestBuilder;

    #[test]
    fn test_tap_suite_builder() {
        let tests = vec![TapTestBuilder::new()
                         .name("Example TAP test")
                         .passed(true)
                         .finalize()];

        let tap_suite_from_builder = TapSuiteBuilder::new()
            .name("Example TAP test suite")
            .tests(tests)
            .finalize();

        let tap_suite_from_scratch = TapSuite {
            name: "Example TAP test suite".to_string(),
            tests: vec![TapTestBuilder::new()
                        .name("Example TAP test")
                        .passed(true)
                        .finalize()],
        };

        assert_eq!(tap_suite_from_builder, tap_suite_from_scratch);
    }
}
