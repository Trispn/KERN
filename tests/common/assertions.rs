use std::fmt::Debug;

#[derive(Debug)]
pub struct AssertionResult {
    pub success: bool,
    pub message: Option<String>,
    pub file: &'static str,
    pub line: u32,
    pub column: u32,
}

impl AssertionResult {
    pub fn new(success: bool, message: Option<String>, file: &'static str, line: u32, column: u32) -> Self {
        Self {
            success,
            message,
            file,
            line,
            column,
        }
    }

    pub fn pass() -> Self {
        Self::new(true, None, "", 0, 0)
    }

    pub fn fail(message: String) -> Self {
        Self::new(false, Some(message), "", 0, 0)
    }
}

pub fn assert_equal<T: PartialEq + Debug>(actual: T, expected: T, message: &str) -> AssertionResult {
    let location = std::panic::Location::caller();
    
    if actual == expected {
        AssertionResult::new(
            true,
            Some(format!("Assertion passed: {:?} == {:?}", actual, expected)),
            location.file(),
            location.line(),
            location.column(),
        )
    } else {
        AssertionResult::new(
            false,
            Some(format!("{} - Expected: {:?}, Actual: {:?}", message, expected, actual)),
            location.file(),
            location.line(),
            location.column(),
        )
    }
}

pub fn assert_true(condition: bool, message: &str) -> AssertionResult {
    let location = std::panic::Location::caller();
    
    if condition {
        AssertionResult::new(
            true,
            Some("Assertion passed: condition is true".to_string()),
            location.file(),
            location.line(),
            location.column(),
        )
    } else {
        AssertionResult::new(
            false,
            Some(format!("{} - Expected: true, Actual: false", message)),
            location.file(),
            location.line(),
            location.column(),
        )
    }
}

pub fn assert_false(condition: bool, message: &str) -> AssertionResult {
    let location = std::panic::Location::caller();
    
    if !condition {
        AssertionResult::new(
            true,
            Some("Assertion passed: condition is false".to_string()),
            location.file(),
            location.line(),
            location.column(),
        )
    } else {
        AssertionResult::new(
            false,
            Some(format!("{} - Expected: false, Actual: true", message)),
            location.file(),
            location.line(),
            location.column(),
        )
    }
}

pub fn assert_raises<F, T, E>(operation: F, expected_error: E, message: &str) -> AssertionResult
where
    F: FnOnce() -> Result<T, E>,
    E: PartialEq + Debug,
{
    let location = std::panic::Location::caller();
    
    match operation() {
        Ok(_) => AssertionResult::new(
            false,
            Some(format!("{} - Expected error but operation succeeded", message)),
            location.file(),
            location.line(),
            location.column(),
        ),
        Err(actual_error) => {
            if actual_error == expected_error {
                AssertionResult::new(
                    true,
                    Some("Assertion passed: expected error occurred".to_string()),
                    location.file(),
                    location.line(),
                    location.column(),
                )
            } else {
                AssertionResult::new(
                    false,
                    Some(format!("{} - Expected error: {:?}, Actual error: {:?}", message, expected_error, actual_error)),
                    location.file(),
                    location.line(),
                    location.column(),
                )
            }
        }
    }
}

pub fn assert_not_equal<T: PartialEq + Debug>(actual: T, expected: T, message: &str) -> AssertionResult {
    let location = std::panic::Location::caller();
    
    if actual != expected {
        AssertionResult::new(
            true,
            Some("Assertion passed: values are not equal".to_string()),
            location.file(),
            location.line(),
            location.column(),
        )
    } else {
        AssertionResult::new(
            false,
            Some(format!("{} - Expected values to be different, but both are: {:?}", message, expected)),
            location.file(),
            location.line(),
            location.column(),
        )
    }
}

pub fn assert_contains<T: PartialEq + Debug>(collection: &[T], item: &T, message: &str) -> AssertionResult {
    let location = std::panic::Location::caller();
    
    if collection.contains(item) {
        AssertionResult::new(
            true,
            Some("Assertion passed: item found in collection".to_string()),
            location.file(),
            location.line(),
            location.column(),
        )
    } else {
        AssertionResult::new(
            false,
            Some(format!("{} - Item {:?} not found in collection", message, item)),
            location.file(),
            location.line(),
            location.column(),
        )
    }
}

pub fn assert_none<T>(option: Option<T>, message: &str) -> AssertionResult {
    let location = std::panic::Location::caller();
    
    match option {
        None => AssertionResult::new(
            true,
            Some("Assertion passed: option is None".to_string()),
            location.file(),
            location.line(),
            location.column(),
        ),
        Some(_) => AssertionResult::new(
            false,
            Some(format!("{} - Expected None but got Some value", message)),
            location.file(),
            location.line(),
            location.column(),
        ),
    }
}

pub fn assert_some<T: Debug>(option: Option<T>, message: &str) -> AssertionResult {
    let location = std::panic::Location::caller();
    
    match option {
        Some(value) => AssertionResult::new(
            true,
            Some(format!("Assertion passed: option contains {:?}", value)),
            location.file(),
            location.line(),
            location.column(),
        ),
        None => AssertionResult::new(
            false,
            Some(format!("{} - Expected Some value but got None", message)),
            location.file(),
            location.line(),
            location.column(),
        ),
    }
}

#[macro_export]
macro_rules! assert_equal_with_tolerance {
    ($actual:expr, $expected:expr, $tolerance:expr, $message:expr) => {
        {
            let diff = ($actual - $expected).abs();
            if diff <= $tolerance {
                AssertionResult::new(
                    true,
                    Some(format!("Assertion passed: {} is within tolerance", stringify!($actual))),
                    file!(),
                    line!(),
                    column!(),
                )
            } else {
                AssertionResult::new(
                    false,
                    Some(format!("{} - Expected: {}, Actual: {}, Tolerance: {}", 
                        $message, $expected, $actual, $tolerance)),
                    file!(),
                    line!(),
                    column!(),
                )
            }
        }
    };
}
