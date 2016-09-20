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
///     .diagnostics(vec!["Something something something".to_string()])
///     .finalize();
/// ```
#[derive(Debug)]
pub struct TapTestBuilder {
    name: String,
    passed: bool,
    diagnostics: Vec<String>,
}

impl TapTestBuilder {
    /// Produce a blank `TapTest` (the default is a passing test)
    pub fn new() -> TapTestBuilder {
        TapTestBuilder { name: "".to_string(), passed: true, diagnostics: vec![] }
    }
    /// Set test name
    pub fn name(&mut self, s: &str) -> &mut TapTestBuilder {
        self.name = s.to_string();
        self
    }
    /// Set passed status
    pub fn passed(&mut self, status: bool) -> &mut TapTestBuilder {
        self.passed = status;
        self
    }
    /// Set diagnostics. This can be any number of lines.
    pub fn diagnostics(&mut self, comments: Vec<String>) -> &mut TapTestBuilder {
        self.diagnostics = comments;
        self
    }
    /// Produce the actual `TapTest` object.
    pub fn finalize(&self) -> TapTest {
        TapTest {
            name: self.name.to_string(),
            passed: self.passed,
            diagnostics: self.diagnostics.clone(),
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
            .diagnostics(vec!["Doing fine".to_string()])
            .finalize();

        let tap_test_from_scratch = TapTest {
            name: "Panda".to_string(),
            passed: true,
            diagnostics: vec!["Doing fine".to_string()],
        };

        assert_eq!(tap_test_from_builder, tap_test_from_scratch);
    }

}
