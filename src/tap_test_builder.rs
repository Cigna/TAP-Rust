use tap_test::TapTest;

/// Coordinator for construction of `TapTest` objects using the builder pattern.
///
/// # Examples
///
/// ```
/// use tap::tap_test_builder::TapTestBuilder;
///
/// let tap_test = TapTestBuilder::new()
///     .name("Panda test")
///     .passed(true)
///     .diagnostics(vec!["Something something something"])
///     .finalize();
/// ```
#[derive(Debug)]
pub struct TapTestBuilder {
    name: Option<String>,
    passed: Option<bool>,
    diagnostics: Option<Vec<String>>,
}

impl TapTestBuilder {
    /// Produce a blank `TapTest` (the default is a passing test)
    pub fn new() -> TapTestBuilder {
        TapTestBuilder {
            name: None,
            passed: None,
            diagnostics: None
        }
    }
    /// Set test name
    pub fn name<S: Into<String>>(&mut self, s: S) -> &mut TapTestBuilder {
        self.name = Some(s.into());
        self
    }
    /// Set passed status
    pub fn passed(&mut self, status: bool) -> &mut TapTestBuilder {
        self.passed = Some(status);
        self
    }
    /// Set diagnostics. This can be any number of lines.
    pub fn diagnostics(&mut self, comments: Vec<&str>) -> &mut TapTestBuilder {
        self.diagnostics = Some(comments.iter().map(|s| s.to_string()).collect());
        self
    }
    /// Produce the configured `TapTest` object. Panics if you don't pass a passed status.
    pub fn finalize(&mut self) -> TapTest {
        TapTest {
            name: self.name.take().unwrap_or("A test has no name".to_string()),
            passed: self.passed.take().expect("You build a test but didn't say whether or not it passed"),
            diagnostics: self.diagnostics.take().unwrap_or(Vec::new()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::TapTestBuilder;
    use tap_test::TapTest;
    
    #[test]
    fn test_tap_test_builder() {
        let tap_test_from_builder = TapTestBuilder::new()
            .name("Panda")
            .passed(true)
            .diagnostics(vec!["Doing fine"])
            .finalize();

        let tap_test_from_scratch = TapTest {
            name: "Panda".to_string(),
            passed: true,
            diagnostics: vec!["Doing fine".to_string()],
        };

        assert_eq!(tap_test_from_builder, tap_test_from_scratch);
    }

    #[test]
    fn test_tap_test_builder_with_no_name() {
        let bad_tap_test = TapTestBuilder::new()
            .passed(true)
            .finalize();

        let expected = "A test has no name";
        assert_eq!(bad_tap_test.name, expected);
    }

    #[test]
    #[should_panic]
    fn test_tap_test_builder_with_no_passed_status() {
        TapTestBuilder::new()
            .name("This should break")
            .finalize();
    }
}
