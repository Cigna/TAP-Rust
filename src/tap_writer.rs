use tap_test::TapTest;

/// A named TAP stream writer. This will print directly to STDOUT as you call methods. No waiting.
pub struct TapWriter {
    /// TAP stream name
    pub name: String,
}

impl TapWriter {
    /// Make me a new one from a name. Don't leave the name blank as it improves clarity.
    pub fn new(name: &str) -> TapWriter {
        TapWriter {
            name: name.to_string(),
        }
    }

    /// Print out the plan like "1..5". If you don't know the plan ahead of time, it can come at the very end.
    pub fn plan(&self, start: i32, finish: i32) {
        println!("{}..{}", start, finish);
    }

    /// Print the suite name as a diagnostic line. Surrounded by blank diagnostic lines because pretty.
    pub fn name(&self) {
        self.diagnostic("");
        self.diagnostic(&self.name);
        self.diagnostic("");
    }

    /// Emit a passing test line.
    pub fn ok(&self, test_number: i32, message: &str) {
        println!("ok {} {}", test_number, message);
    }

    /// Emit a failing test line.
    pub fn not_ok(&self, test_number: i32, message: &str) {
        println!("not ok {} {}", test_number, message);
    }

    /// Emit a diagnostic message. Prefaced with a #.
    pub fn diagnostic(&self, message: &str) {
        println!("# {}", message);
    }

    /// Emergency stop! This should be the last thing in the TAP stream. Nothing may come after it.
    pub fn bail_out(&self) {
        self.bail_out_with_message("");
    }

    /// In case you want to bail out with a message. Please use this instead of plain `bail_out`.
    pub fn bail_out_with_message(&self, message: &str) {
        println!("Bail out! {}", message);
    }
}
