use tap_test::TapTest;

pub struct TapWriter {
    name: String,
}

impl TapWriter {
    pub fn new(name: &str) -> TapWriter {
        TapWriter {
            name: name.to_string(),
        }
    }

    pub fn plan(&self, start: i32, finish: i32) {
        println!("{}..{}", start, finish);
    }

    pub fn name(&self) {
        self.diagnostic("");
        self.diagnostic(&self.name);
        self.diagnostic("");
    }
    
    pub fn ok(&self, test_number: i32, message: &str) {
        println!("ok {} {}", test_number, message);
    }

    pub fn not_ok(&self, test_number: i32, message: &str) {
        println!("not ok {} {}", test_number, message);
    }

    pub fn diagnostic(&self, message: &str) {
        println!("# {}", message);
    }

    pub fn bail_out(&self) {
        println!("Bail out!");
    }

    pub fn bail_out_with_message(&self, message: &str) {
        println!("Bail out! {}", message);
    }
}
