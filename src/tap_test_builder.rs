use tap_test::TapTest;

/// Coordinator for construction of TapTest objects using the builder pattern.
#[derive(Debug)]
pub struct TapTestBuilder {
    name: String,
    passed: bool,
    commentary: Vec<String>,
}

impl TapTestBuilder {
    /// Produce a blank TapTest (the default is a passing test)
    pub fn new() -> TapTestBuilder {
        TapTestBuilder { name: "".to_string(), passed: true, commentary: vec![] }
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
    /// Set commentary. This can be any number of lines.
    pub fn commentary(&mut self, comments: Vec<String>) -> &mut TapTestBuilder {
        self.commentary = comments;
        self
    }
    /// Produce the actual TapTest object.
    pub fn finalize(&self) -> TapTest {
        TapTest {
            name: self.name.to_string(),
            passed: self.passed,
            commentary: self.commentary.iter().map(|c| c.to_string() ).collect::<Vec<String>>(),
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
            .commentary(vec!["Doing fine".to_string()])
            .finalize();

        let tap_test_from_scratch = TapTest {
            name: "Panda".to_string(),
            passed: true,
            commentary: vec!["Doing fine".to_string()],
        };

        let expected = format!("{:?}", tap_test_from_builder);
        let actual = format!("{:?}", tap_test_from_scratch);
        
        assert_eq!(expected, actual);
    }

}
