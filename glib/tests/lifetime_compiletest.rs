//! Compiletests for wrappers with (covariant) lifetime parameters.

#[test]
pub fn test() {
    let t = trybuild2::TestCases::new();

    t.pass("tests/lifetime_compiletest/01-boxed-implicit-drop.rs");
    t.pass("tests/lifetime_compiletest/02-boxed-explicit-drop.rs");
    t.compile_fail_check_sub(
        "tests/lifetime_compiletest/03-boxed-dangling.rs",
        "error[E0505]: cannot move out of `data` because it is borrowed",
    );

    t.pass("tests/lifetime_compiletest/04-boxed-static-value.rs");
    t.compile_fail("tests/lifetime_compiletest/05-boxed-dangling-value.rs");
    t.compile_fail("tests/lifetime_compiletest/06-boxed-invariant.rs");
    t.compile_fail("tests/lifetime_compiletest/07-boxed-nonstatic-from-value.rs");
}
