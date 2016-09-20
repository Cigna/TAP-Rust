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
#[derive(Debug)]
pub struct TapSuiteBuilder {
    /// Name of test suite
    pub name: String,
    /// Vector of type `Vec<TapTest>` which holds the actual tests
    pub tests: Vec<TapTest>,
}

impl TapSuiteBuilder {
    /// Produce a new builder object
    pub fn new() -> TapSuiteBuilder {
        TapSuiteBuilder { name: "".to_string(), tests: vec![] }
    }
    /// Set the name
    pub fn name(&mut self, s: &str) -> &mut TapSuiteBuilder {
        self.name = s.to_string();
        self
    }
    /// Set the tests
    pub fn tests(&mut self, test_vec: Vec<TapTest>) -> &mut TapSuiteBuilder {
        self.tests = test_vec;
        self
    }
    /// Produce the configured `TapSuite` object
    pub fn finalize(&mut self) -> TapSuite {
        TapSuite {
            name: self.name.to_string(),
            tests: self.tests.iter().map(|test| {
                TapTest {
                    name: test.name.to_string(),
                    passed: test.passed,
                    diagnostics: test.diagnostics.clone(),
                }
            }).collect()
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
