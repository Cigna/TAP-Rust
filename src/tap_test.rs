#[derive(Debug)]
pub struct TapTest {
    pub name: String,
    pub passed: bool,
    pub commentary: Vec<String>,
}

impl TapTest {
    pub fn ok_string(&self) -> String {
        let result = if self.passed {
            "ok"
        } else {
            "not ok"
        };

        result.to_string()
    }

    pub fn status_line(&self, test_number: i64) -> String {
        format!("{} {} {}", self.ok_string(), test_number, self.name)
    }

    pub fn tap(&self, test_number: i64) -> Vec<String> {
        let mut lines = vec![self.status_line(test_number)];
        lines.extend(self.commentary.iter().cloned());

        lines
    }
}
