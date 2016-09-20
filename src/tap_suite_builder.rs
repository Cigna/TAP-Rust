use tap_suite::TapSuite;
use tap_test::TapTest;

pub struct TapSuiteBuilder {
    pub name: String,
    pub tests: Vec<TapTest>,
}

impl TapSuiteBuilder {
    fn new() -> TapSuiteBuilder {
        TapSuiteBuilder { name: "".to_string(), tests: vec![] }
    }

    fn name(&mut self, s: &str) -> &mut TapSuiteBuilder {
        self.name = s.to_string();
        self
    }

    fn tests(&mut self, test_vec: Vec<TapTest>) -> &mut TapSuiteBuilder {
        self.tests = test_vec;
        self
    }

    fn finalize(&mut self) -> TapSuite {
        TapSuite {
            name: self.name.to_string(),
            tests: self.tests.iter().map(|test| {
                let diagnostics = test.diagnostics
                    .iter()
                    .map(|s| s.to_string())
                    .collect::<Vec<String>>();

                TapTest {
                    name: test.name.to_string(),
                    passed: test.passed,
                    diagnostics: diagnostics,
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

        let actual = format!("{:?}", tap_suite_from_builder);
        let expected = format!("{:?}", tap_suite_from_scratch);

        assert_eq!(expected, actual);
    }
}
