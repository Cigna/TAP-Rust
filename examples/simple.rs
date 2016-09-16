
extern crate tap;

use tap::tap_test_builder::TapTestBuilder;
use tap::tap_suite::TapSuite;

fn main() {
    let passing_test = TapTestBuilder::new()
        .name("Panda Bamboo")
        .passed(true)
        .finalize();

    let failing_test = TapTestBuilder::new()
        .name("Curry Noodle")
        .passed(false)
        .commentary(vec!["Tree".to_string(), "Flower".to_string()])
        .finalize();

    let tap_suite = TapSuite {
        tests: vec![passing_test, failing_test],
    };

    tap_suite.print();
}
