use tap_test::TapTest;

#[derive(Debug)]
pub struct TapTestBuilder {
    name: String,
    passed: bool,
    commentary: Vec<String>,
}

impl TapTestBuilder {
    pub fn new() -> TapTestBuilder {
        TapTestBuilder { name: "".to_string(), passed: true, commentary: vec![] }
    }

    pub fn name(&mut self, s: &str) -> &mut TapTestBuilder {
        self.name = s.to_string();
        self
    }

    pub fn passed(&mut self, status: bool) -> &mut TapTestBuilder {
        self.passed = status;
        self
    }

    pub fn commentary(&mut self, comments: Vec<String>) -> &mut TapTestBuilder {
        self.commentary = comments;
        self
    }
    
    pub fn finalize(&self) -> TapTest {
        TapTest {
            name: self.name.to_string(),
            passed: self.passed,
            commentary: self.commentary.iter().map(|c| c.to_string() ).collect::<Vec<String>>(),
        }
    }
}
