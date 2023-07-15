use adder;

/*
    INTEGRATION TESTS IN RUST

    If you create a tests folder, any file within it will be treated as a separate
    test file. For example ---->

    tests---
        test_file_one.rs
        test_file_two.rs
        test_file_three.rs

    All of the above files will be picked up by the test runner as tests to be run.

    What if there were common functionality we wanted to share between those tests?

    tests---
        common.rs
        test_file_one.rs
        test_file_two.rs
        test_file_three.rs

    If we did it that way, common.rs would show up as an integration test file and it
    would not be proper behavior. Instead, we can use a folder inside the tests directory.

    tests---
        common---
            mods.rs
        test_file_one.rs
        test_file_two.rs
        test_file_three.rs

    Since mods.rs is contained within a subfolder within tests and not within the root of
    that directory itself, it is not parsed and included as part of the suite. We would be able
    to use it by writing:

        mod common;
*/

mod common;

#[test]
fn it_adds_two() {
    assert_eq!(4, adder::add_two(2));
}
