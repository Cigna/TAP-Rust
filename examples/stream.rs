
extern crate tap_rust;

use tap_rust::tap_writer::TapWriter;

fn main() {
    let writer = TapWriter::new("Example TAP stream");

    // Write the plan out. This can come before or after the test results themselves.
    writer.plan(1, 6);

    // Give me the name as a diagnostic line
    writer.name();

    // Print out some test results
    writer.ok(1, "Panda");
    writer.ok(2, "Bamboo");
    writer.ok(3, "Curry");
    // This one failed, so explain why with a diagnostic line
    writer.not_ok(4, "Noodle"); 
    writer.diagnostic("The above test failed because of XYZ reason");
    writer.ok(5, "Tree");

    // uh oh! something went horribly wrong and we need to stop before
    // we print out the results from test 6!
    writer.bail_out_with_message("Destabilized warp core! Can't continue!");
}
