mod util;
use smartstring::alias::String as SmartString;
use util::*;

#[test]
fn smartstring() -> TestResult {
    test_default_generated_schema::<SmartString>("smartstring")
}
