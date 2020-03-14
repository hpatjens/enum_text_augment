#[test]
fn tests() {
    let t = trybuild::TestCases::new();
    t.pass("tests/01-basic.rs");
    t.pass("tests/02-override.rs");
}
