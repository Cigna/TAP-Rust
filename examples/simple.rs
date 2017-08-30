
extern crate tap_rust;

use tap_rust::tap_test_builder::TapTestBuilder;
use tap_rust::tap_suite_builder::TapSuiteBuilder;

fn main() {
    // Make some tests
    let passing_test = TapTestBuilder::new()
        .name("Panda Bamboo")
        .passed(true)
        .finalize();

    let failing_test = TapTestBuilder::new()
        .name("Curry Noodle")
        .passed(false)
        .diagnostics(vec!["Tree", "Flower"])
        .finalize();

    // Build the suite
    let tap_suite = TapSuiteBuilder::new()
        .name("Example TAP Suite")
        .tests(vec![passing_test, failing_test])
        .finalize();
    
    tap_suite.print();
}
