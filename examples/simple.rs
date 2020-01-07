extern crate testanything;

use testanything::tap_suite_builder::TapSuiteBuilder;
use testanything::tap_test_builder::TapTestBuilder;

use std::io;

fn main() {
    // Make some tests
    let passing_test = TapTestBuilder::new()
        .name("Panda Bamboo")
        .passed(true)
        .finalize();

    let failing_test = TapTestBuilder::new()
        .name("Curry Noodle")
        .passed(false)
        .diagnostics(&vec!["Tree", "Flower"])
        .finalize();

    // Build the suite
    let tap_suite = TapSuiteBuilder::new()
        .name("Example TAP Suite")
        .tests(vec![passing_test, failing_test])
        .finalize();

    match tap_suite.print(io::stdout().lock()) {
        Ok(_) => {}
        Err(reason) => eprintln!("{}", reason),
    }
}
