use nu_test_support::{nu, pipeline};

#[test]
fn do_works_single_line() {
    let actual = nu!(
        cwd: "tests/fixtures/formats", pipeline(
        r#"
        echo [1 2 3] | do { sum } 
        "#
    ));

    assert_eq!(actual, "6");
}