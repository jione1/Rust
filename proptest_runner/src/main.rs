extern crate proptest;

use proptest::test_runner::{Config, FileFailurePersistence, TestError, TestRunner};

fn some_function(v: i32) {

    assert!(v <= 500);
}

fn main() {
    let mut runner = TestRunner::new(Config {
        failure_persistence: Some(Box::new(FileFailurePersistence::Off)), .. Config::default()
    });
    let result = runner.run(&(0..10000i32), |&v| {some_function(v); 
        Ok(())
    });
    match result {
        Err(TestError::Fail(_, value)) => {
            println!("Found minimal failing case: {}", value);
            assert_eq!(501, value);
        },
        result => panic!("Unexpected result: {:?}", result),
    }
}