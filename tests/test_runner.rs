use std::collections::HashMap;
use std::time::Instant;

#[derive(Debug, Clone)]
pub struct TestResult {
    pub name: String,
    pub status: TestStatus,
    pub duration: std::time::Duration,
    pub message: Option<String>,
}

impl TestResult {
    /// Create a TestResult from simplified test parameters
    /// This matches the calling convention used in main_test_runner.rs
    pub fn new(
        passed: bool,
        message: Option<String>,
        test_name: &str,
        _line: usize,
        _column: usize,
    ) -> Self {
        Self {
            name: test_name.to_string(),
            status: if passed {
                TestStatus::Pass
            } else {
                TestStatus::Fail
            },
            duration: std::time::Duration::new(0, 0), // Duration is set later by TestRunner
            message,
        }
    }
}

#[derive(Debug, Clone)]
pub enum TestStatus {
    Pass,
    Fail,
    Skip,
}

pub struct TestRunner {
    pub tests: Vec<(&'static str, fn() -> TestResult)>,
}

impl TestRunner {
    pub fn new() -> Self {
        Self { tests: Vec::new() }
    }

    pub fn add_test(&mut self, name: &'static str, test_fn: fn() -> TestResult) {
        self.tests.push((name, test_fn));
    }

    pub fn run_all(&self) -> Vec<TestResult> {
        let mut results = Vec::new();

        for (name, test_fn) in &self.tests {
            println!("Running test: {}", name);
            let start_time = Instant::now();

            let result = test_fn();
            let duration = start_time.elapsed();

            results.push(TestResult {
                name: result.name,
                status: result.status,
                duration,
                message: result.message,
            });
        }

        results
    }

    pub fn run_with_report(&self) -> TestReport {
        let results = self.run_all();
        let mut report = TestReport::new(results);
        report.print_summary();
        report
    }
}

#[derive(Debug)]
pub struct TestReport {
    pub results: Vec<TestResult>,
    pub total: usize,
    pub passed: usize,
    pub failed: usize,
    pub skipped: usize,
    pub duration: std::time::Duration,
}

impl TestReport {
    pub fn new(results: Vec<TestResult>) -> Self {
        let start = results
            .iter()
            .map(|r| r.duration)
            .min()
            .unwrap_or(std::time::Duration::new(0, 0));
        let end = results
            .iter()
            .map(|r| r.duration)
            .max()
            .unwrap_or(std::time::Duration::new(0, 0));

        let total_duration = end.saturating_sub(start);

        let passed = results
            .iter()
            .filter(|r| matches!(r.status, TestStatus::Pass))
            .count();
        let failed = results
            .iter()
            .filter(|r| matches!(r.status, TestStatus::Fail))
            .count();
        let skipped = results
            .iter()
            .filter(|r| matches!(r.status, TestStatus::Skip))
            .count();

        Self {
            results: results.clone(),
            total: results.len(),
            passed,
            failed,
            skipped,
            duration: total_duration,
        }
    }

    pub fn print_summary(&self) {
        println!("\n=== Test Results Summary ===");
        println!("Total: {}", self.total);
        println!("Passed: {}", self.passed);
        println!("Failed: {}", self.failed);
        println!("Skipped: {}", self.skipped);
        println!("Duration: {:?}", self.duration);

        if self.failed > 0 {
            println!("\nFailed tests:");
            for result in &self.results {
                if matches!(result.status, TestStatus::Fail) {
                    println!("  [FAIL] {}: {:?}", result.name, result.message);
                }
            }
        }
    }

    pub fn to_json(&self) -> String {
        let json_results: Vec<HashMap<String, String>> = self
            .results
            .iter()
            .map(|result| {
                let mut map = HashMap::new();
                map.insert("name".to_string(), result.name.clone());

                let status_str = match result.status {
                    TestStatus::Pass => "PASS".to_string(),
                    TestStatus::Fail => "FAIL".to_string(),
                    TestStatus::Skip => "SKIP".to_string(),
                };

                map.insert("status".to_string(), status_str);
                map.insert("duration".to_string(), format!("{:?}", result.duration));

                if let Some(ref msg) = result.message {
                    map.insert("message".to_string(), msg.clone());
                }

                map
            })
            .collect();

        format!("{:#?}", json_results)
    }
}

// Global test runner instance
static mut TEST_RUNNER: Option<TestRunner> = None;

pub fn get_test_runner() -> &'static mut TestRunner {
    unsafe {
        if TEST_RUNNER.is_none() {
            TEST_RUNNER = Some(TestRunner::new());
        }
        TEST_RUNNER.as_mut().unwrap()
    }
}

#[macro_export]
macro_rules! test {
    ($name:ident, $body:block) => {
        #[test]
        fn $name() {
            let start_time = std::time::Instant::now();

            let result = std::panic::catch_unwind(|| {
                $body
                true
            });

            let duration = start_time.elapsed();

            match result {
                Ok(true) => {
                    println!("[PASS] {} (Duration: {:?})", stringify!($name), duration);
                }
                Ok(false) => {
                    println!("[FAIL] {} (Duration: {:?})", stringify!($name), duration);
                    panic!("Test {} failed", stringify!($name));
                }
                Err(_) => {
                    println!("[FAIL] {} (Duration: {:?}) - Panic occurred", stringify!($name), duration);
                    panic!("Test {} panicked", stringify!($name));
                }
            }
        }
    };
}
