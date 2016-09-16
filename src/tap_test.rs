
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


#[cfg(test)]
mod tests {
    use super::TapTest;
    
    #[test]
    fn test_tap_test_status_string() {
        let tap_test_passing = TapTest {
            name: "Panda".to_string(),
            passed: true,
            commentary: vec!["Doing fine".to_string()],
        };

        let expected_passing = "ok";
        let actual_passing = tap_test_passing.ok_string();
        
        assert_eq!(expected_passing, actual_passing);

        let tap_test_failing = TapTest {
            name: "Panda".to_string(),
            passed: false,
            commentary: vec!["Doing fine".to_string()],
        };

        let expected_failing = "not ok";
        let actual_failing = tap_test_failing.ok_string();
        
        assert_eq!(expected_failing, actual_failing);        
    }

    #[test]
    fn test_format_first_line() {
        let tap_test_passing = TapTest {
            name: "Panda".to_string(),
            passed: true,
            commentary: vec!["Doing fine".to_string()],
        };

        let expected_passing = "ok 42 Panda";
        let actual_passing = tap_test_passing.status_line(42);

        assert_eq!(expected_passing, actual_passing);

        let tap_test_failing = TapTest {
            name: "Panda".to_string(),
            passed: false,
            commentary: vec!["Doing fine".to_string()],
        };

        let expected_failing = "not ok 42 Panda";
        let actual_failing = tap_test_failing.status_line(42);
        
        assert_eq!(expected_failing, actual_failing);        

    }

    #[test]
    fn test_tap_lines() {
        let tap_test_passing = TapTest {
            name: "Panda".to_string(),
            passed: true,
            commentary: vec!["Doing fine".to_string()],
        };

        let expected_passing = vec!["ok 42 Panda", "# Doing fine"];
        let actual_passing = tap_test_passing.tap(42);

        assert_eq!(expected_passing, actual_passing);        
    }

    #[test]
    fn test_format_commentary() {
        let tap_test_passing = TapTest {
            name: "Panda".to_string(),
            passed: true,
            commentary: vec!["Doing fine".to_string()],
        };

        let expected_passing = "# Doing fine";
        let actual_passing = tap_test_passing.format_commentary(&tap_test_passing.commentary[0]);

        assert_eq!(expected_passing, actual_passing);        
    }
}
