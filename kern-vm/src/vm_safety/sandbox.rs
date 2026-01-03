//! Sandboxed Execution Environment for KERN VM
//! 
//! Implements the sandboxed execution environment as specified in the safety layer.

use std::collections::HashMap;

/// Sandbox policy configuration structure
#[derive(Debug, Clone)]
pub struct SandboxPolicy {
    pub allowed_functions: Vec<String>,
    pub allowed_io_channels: Vec<String>,
    pub max_calls_per_function: HashMap<String, u64>,
}

impl SandboxPolicy {
    pub fn new() -> Self {
        SandboxPolicy {
            allowed_functions: Vec::new(),
            allowed_io_channels: Vec::new(),
            max_calls_per_function: HashMap::new(),
        }
    }

    /// Add an allowed external function to the policy
    pub fn allow_function(&mut self, function_name: &str) {
        if !self.allowed_functions.contains(&function_name.to_string()) {
            self.allowed_functions.push(function_name.to_string());
        }
    }

    /// Add an allowed IO channel to the policy
    pub fn allow_io_channel(&mut self, channel_name: &str) {
        if !self.allowed_io_channels.contains(&channel_name.to_string()) {
            self.allowed_io_channels.push(channel_name.to_string());
        }
    }

    /// Set the maximum number of calls allowed for a function
    pub fn set_max_calls_for_function(&mut self, function_name: &str, max_calls: u64) {
        self.max_calls_per_function.insert(function_name.to_string(), max_calls);
    }

    /// Check if a function is allowed by the policy
    pub fn is_function_allowed(&self, function_name: &str) -> bool {
        self.allowed_functions.contains(&function_name.to_string())
    }

    /// Check if an IO channel is allowed by the policy
    pub fn is_io_channel_allowed(&self, channel_name: &str) -> bool {
        self.allowed_io_channels.contains(&channel_name.to_string())
    }

    /// Check if a function call would exceed the call limit
    pub fn would_exceed_call_limit(&self, function_name: &str, current_calls: u64) -> bool {
        if let Some(&max_calls) = self.max_calls_per_function.get(function_name) {
            current_calls >= max_calls
        } else {
            false // If no limit is set, assume unlimited
        }
    }
}

/// Default sandbox policy that allows no external functions or IO
impl Default for SandboxPolicy {
    fn default() -> Self {
        SandboxPolicy::new()
    }
}

/// Function call tracker to enforce call limits
#[derive(Debug, Clone)]
pub struct FunctionCallTracker {
    pub call_counts: HashMap<String, u64>,
}

impl FunctionCallTracker {
    pub fn new() -> Self {
        FunctionCallTracker {
            call_counts: HashMap::new(),
        }
    }

    /// Record a function call and check if it violates the policy
    pub fn record_call(&mut self, function_name: &str, policy: &SandboxPolicy) -> Result<(), SandboxError> {
        // Check if function is allowed
        if !policy.is_function_allowed(function_name) {
            return Err(SandboxError::FunctionNotAllowed(function_name.to_string()));
        }

        // Increment call count
        let count = self.call_counts.entry(function_name.to_string()).or_insert(0);
        *count += 1;

        // Check if call limit is exceeded
        if policy.would_exceed_call_limit(function_name, *count) {
            return Err(SandboxError::CallLimitExceeded(function_name.to_string()));
        }

        Ok(())
    }

    /// Get the current call count for a function
    pub fn get_call_count(&self, function_name: &str) -> u64 {
        *self.call_counts.get(function_name).unwrap_or(&0)
    }
}

/// IO operation tracker to enforce IO limits
#[derive(Debug, Clone)]
pub struct IoOperationTracker {
    pub io_counts: HashMap<String, u64>,
}

impl IoOperationTracker {
    pub fn new() -> Self {
        IoOperationTracker {
            io_counts: HashMap::new(),
        }
    }

    /// Record an IO operation and check if it violates the policy
    pub fn record_io_operation(&mut self, channel_name: &str, policy: &SandboxPolicy) -> Result<(), SandboxError> {
        // Check if IO channel is allowed
        if !policy.is_io_channel_allowed(channel_name) {
            return Err(SandboxError::IoChannelNotAllowed(channel_name.to_string()));
        }

        // Increment IO count
        let count = self.io_counts.entry(channel_name.to_string()).or_insert(0);
        *count += 1;

        Ok(())
    }

    /// Get the current IO count for a channel
    pub fn get_io_count(&self, channel_name: &str) -> u64 {
        *self.io_counts.get(channel_name).unwrap_or(&0)
    }
}

/// Sandboxed execution environment
#[derive(Debug)]
pub struct SandboxEnvironment {
    pub policy: SandboxPolicy,
    pub function_tracker: FunctionCallTracker,
    pub io_tracker: IoOperationTracker,
}

impl SandboxEnvironment {
    pub fn new(policy: SandboxPolicy) -> Self {
        SandboxEnvironment {
            policy,
            function_tracker: FunctionCallTracker::new(),
            io_tracker: IoOperationTracker::new(),
        }
    }

    /// Execute an external function call within the sandbox
    pub fn execute_external_call(&mut self, function_name: &str) -> Result<(), SandboxError> {
        self.function_tracker.record_call(function_name, &self.policy)
    }

    /// Execute an IO operation within the sandbox
    pub fn execute_io_operation(&mut self, channel_name: &str) -> Result<(), SandboxError> {
        self.io_tracker.record_io_operation(channel_name, &self.policy)
    }

    /// Check if the environment is properly sandboxed
    pub fn is_sandboxed(&self) -> bool {
        // A properly sandboxed environment has at least some restrictions
        !self.policy.allowed_functions.is_empty() || 
        !self.policy.allowed_io_channels.is_empty()
    }

    /// Get the current call count for a function
    pub fn get_function_call_count(&self, function_name: &str) -> u64 {
        self.function_tracker.get_call_count(function_name)
    }

    /// Get the current IO count for a channel
    pub fn get_io_operation_count(&self, channel_name: &str) -> u64 {
        self.io_tracker.get_io_count(channel_name)
    }
}

/// Sandbox errors
#[derive(Debug, Clone, PartialEq)]
pub enum SandboxError {
    FunctionNotAllowed(String),
    IoChannelNotAllowed(String),
    CallLimitExceeded(String),
}

impl std::fmt::Display for SandboxError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SandboxError::FunctionNotAllowed(name) => write!(f, "Function not allowed in sandbox: {}", name),
            SandboxError::IoChannelNotAllowed(name) => write!(f, "IO channel not allowed in sandbox: {}", name),
            SandboxError::CallLimitExceeded(name) => write!(f, "Call limit exceeded for function: {}", name),
        }
    }
}

impl std::error::Error for SandboxError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sandbox_policy() {
        let mut policy = SandboxPolicy::new();
        
        // Test adding allowed functions
        policy.allow_function("print");
        policy.allow_function("read_file");
        assert!(policy.is_function_allowed("print"));
        assert!(policy.is_function_allowed("read_file"));
        assert!(!policy.is_function_allowed("system"));

        // Test setting call limits
        policy.set_max_calls_for_function("print", 5);
        assert!(policy.would_exceed_call_limit("print", 5));
        assert!(!policy.would_exceed_call_limit("print", 4));
    }

    #[test]
    fn test_function_call_tracker() {
        let mut policy = SandboxPolicy::new();
        policy.allow_function("test_func");
        policy.set_max_calls_for_function("test_func", 3);

        let mut tracker = FunctionCallTracker::new();

        // Test successful calls
        for _ in 0..3 {
            assert!(tracker.record_call("test_func", &policy).is_ok());
        }

        // Test call that exceeds limit
        assert_eq!(tracker.record_call("test_func", &policy),
                   Err(SandboxError::CallLimitExceeded("test_func".to_string())));

        // Test calling a function that's not allowed
        assert_eq!(tracker.record_call("forbidden_func", &policy),
                   Err(SandboxError::FunctionNotAllowed("forbidden_func".to_string())));
    }

    #[test]
    fn test_io_operation_tracker() {
        let mut policy = SandboxPolicy::new();
        policy.allow_io_channel("stdout");
        
        let mut tracker = IoOperationTracker::new();

        // Test successful IO operations
        for _ in 0..5 {
            assert!(tracker.record_io_operation("stdout", &policy).is_ok());
        }

        // Test IO operation on a channel that's not allowed
        assert_eq!(tracker.record_io_operation("network", &policy),
                   Err(SandboxError::IoChannelNotAllowed("network".to_string())));
    }

    #[test]
    fn test_sandbox_environment() {
        let mut policy = SandboxPolicy::new();
        policy.allow_function("print");
        policy.set_max_calls_for_function("print", 2);
        policy.allow_io_channel("stdout");

        let mut sandbox = SandboxEnvironment::new(policy);

        // Test external function calls
        assert!(sandbox.execute_external_call("print").is_ok());
        assert!(sandbox.execute_external_call("print").is_ok());
        assert_eq!(sandbox.execute_external_call("print"),
                   Err(SandboxError::CallLimitExceeded("print".to_string())));

        // Test IO operations
        assert!(sandbox.execute_io_operation("stdout").is_ok());
        assert!(sandbox.execute_io_operation("stdout").is_ok());

        // Test calling a function that's not allowed
        assert_eq!(sandbox.execute_external_call("system"),
                   Err(SandboxError::FunctionNotAllowed("system".to_string())));

        // Verify call counts
        assert_eq!(sandbox.get_function_call_count("print"), 3); // Includes the failed call
        assert_eq!(sandbox.get_io_operation_count("stdout"), 2);
    }
}