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
        let formatted_commentary = self.commentary
            .iter()
            .map(|comment| self.format_commentary(comment))
            .collect::<Vec<String>>();
        
        lines.extend(formatted_commentary.iter().cloned());

        lines
    }

    pub fn format_commentary(&self, line: &str) -> String {
        format!("# {}", line)
    }
}
