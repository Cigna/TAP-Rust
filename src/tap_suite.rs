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
