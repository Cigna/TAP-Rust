mod tap_test;
mod tap_test_builder;

use tap_test::TapTest;

#[derive(Debug)]
struct TapSuite {
    tests: Vec<TapTest>,
}

impl TapSuite {
    fn lines(&self) -> Vec<String> {
        let mut all_lines = Vec::new();

        for (i, test) in self.tests.iter().enumerate() {
            let index = i as i64;
            let tap = test.tap(index);
            all_lines.extend(tap.iter().cloned());
        }

        all_lines
    }

    fn print(&self) {
        for line in self.lines() {
            println!("{}", line);
        }
    }
}

#[cfg(test)]
mod tests {
    //use super::{TapTest, TapSuite, TapTestBuilder};
    use tap_test::TapTest;
    use tap_test_builder::TapTestBuilder;
    
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
