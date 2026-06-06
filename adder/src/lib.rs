/*pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]//indicates that this is a test function
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);//assert_eq! is a macro that checks if the two arguments are equal. If they are not, the test will fail and print the values of the arguments.
    }
}
*/
/*
pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exploration() {//This is a test function that checks if the add function works correctly. It will pass if the result of add(2, 2) is equal to 4, and fail otherwise.
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn another() {//This is another test function that will always fail because it contains a panic! macro. The panic! macro is used to indicate that something went wrong and the test should fail.
        panic!("Make this test fail");
    }
}*/


//Testing equality with asert_eq! and assert_ne!
//asert_eq! checks if two values are equal, and assert_ne! checks if two values are not equal. If the condition is not met, the test will fail and print the values of the arguments.


// ==========Controlling how tests are run==========
//cargo test runs all Rust tests, and they run in parallel by default.
//Use cargo test -- --test-threads=1 to run tests one by one.
//println! output is hidden for passing tests; use --show-output to display it.
//You can run specific tests by name, e.g., cargo test one_hundred.
//Use #[ignore] for slow tests and run them later with cargo test -- --ignored.

//test output from failed tests is shown automatically to help with debugging.
//Partial test names can be used to run multiple related tests, e.g., cargo test add.
//Ignored tests are useful for long-running or resource-intensive test cases.



//===========Test organization==========
/*Rust has two types of tests: Unit Tests and Integration Tests.
Unit tests are written inside the src files and test individual functions/modules in isolation.
The #[cfg(test)] attribute ensures test code is compiled only when running cargo test.
Rust allows testing of private functions using use super::*.
Integration tests are placed in the tests/ directory and use only the public API.
Each file in the tests/ directory is compiled as a separate crate.
Shared helper code for integration tests should be placed in tests/common/mod.rs.
Unit tests verify individual components, while integration tests verify that multiple components work together correctly.*/