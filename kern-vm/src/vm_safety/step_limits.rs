//! Execution Step Limits Implementation for KERN VM
//! 
//! Implements the execution step counting and limiting system as specified in the safety layer.

/// Execution limits configuration structure
#[derive(Debug, Clone)]
pub struct ExecutionLimits {
    pub max_steps: u64,
    pub max_rule_invocations: u64,
    pub max_loop_iterations: u64,
}

impl ExecutionLimits {
    pub fn new(max_steps: u64, max_rule_invocations: u64, max_loop_iterations: u64) -> Self {
        ExecutionLimits {
            max_steps,
            max_rule_invocations,
            max_loop_iterations,
        }
    }

    /// Default execution limits for testing purposes
    pub fn default() -> Self {
        ExecutionLimits {
            max_steps: 1_000_000,      // 1 million steps
            max_rule_invocations: 100_000, // 100k rule invocations
            max_loop_iterations: 100_000,  // 100k loop iterations
        }
    }
}

/// Execution step counter for runtime enforcement
#[derive(Debug, Clone)]
pub struct ExecutionCounters {
    pub step_count: u64,
    pub rule_invocation_count: u64,
    pub loop_iteration_count: u64,
}

impl ExecutionCounters {
    pub fn new() -> Self {
        ExecutionCounters {
            step_count: 0,
            rule_invocation_count: 0,
            loop_iteration_count: 0,
        }
    }

    /// Increment step counter and check if limit is exceeded
    pub fn increment_step(&mut self, limits: &ExecutionLimits) -> Result<(), StepLimitError> {
        self.step_count += 1;
        if self.step_count > limits.max_steps {
            return Err(StepLimitError::StepLimitExceeded);
        }
        Ok(())
    }

    /// Increment rule invocation counter and check if limit is exceeded
    pub fn increment_rule_invocation(&mut self, limits: &ExecutionLimits) -> Result<(), StepLimitError> {
        self.rule_invocation_count += 1;
        if self.rule_invocation_count > limits.max_rule_invocations {
            return Err(StepLimitError::RuleLimitExceeded);
        }
        Ok(())
    }

    /// Increment loop iteration counter and check if limit is exceeded
    pub fn increment_loop_iteration(&mut self, limits: &ExecutionLimits) -> Result<(), StepLimitError> {
        self.loop_iteration_count += 1;
        if self.loop_iteration_count > limits.max_loop_iterations {
            return Err(StepLimitError::LoopLimitExceeded);
        }
        Ok(())
    }

    /// Check if any counter exceeds its limit
    pub fn exceeds_limit(&self, limits: &ExecutionLimits) -> Option<StepLimitError> {
        if self.step_count > limits.max_steps {
            return Some(StepLimitError::StepLimitExceeded);
        }
        if self.rule_invocation_count > limits.max_rule_invocations {
            return Some(StepLimitError::RuleLimitExceeded);
        }
        if self.loop_iteration_count > limits.max_loop_iterations {
            return Some(StepLimitError::LoopLimitExceeded);
        }
        None
    }
}

/// Step limit errors
#[derive(Debug, Clone, PartialEq)]
pub enum StepLimitError {
    StepLimitExceeded,
    RuleLimitExceeded,
    LoopLimitExceeded,
}

impl std::fmt::Display for StepLimitError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StepLimitError::StepLimitExceeded => write!(f, "Execution step limit exceeded"),
            StepLimitError::RuleLimitExceeded => write!(f, "Rule invocation limit exceeded"),
            StepLimitError::LoopLimitExceeded => write!(f, "Loop iteration limit exceeded"),
        }
    }
}

impl std::error::Error for StepLimitError {}

/// Step limiter that combines limits and counters
#[derive(Debug)]
pub struct StepLimiter {
    pub limits: ExecutionLimits,
    pub counters: ExecutionCounters,
}

impl StepLimiter {
    pub fn new(limits: ExecutionLimits) -> Self {
        StepLimiter {
            limits,
            counters: ExecutionCounters::new(),
        }
    }

    /// Increment step counter and enforce limit
    pub fn increment_step(&mut self) -> Result<(), StepLimitError> {
        self.counters.increment_step(&self.limits)
    }

    /// Increment rule invocation counter and enforce limit
    pub fn increment_rule_invocation(&mut self) -> Result<(), StepLimitError> {
        self.counters.increment_rule_invocation(&self.limits)
    }

    /// Increment loop iteration counter and enforce limit
    pub fn increment_loop_iteration(&mut self) -> Result<(), StepLimitError> {
        self.counters.increment_loop_iteration(&self.limits)
    }

    /// Check if any counter exceeds its limit
    pub fn exceeds_limit(&self) -> Option<StepLimitError> {
        self.counters.exceeds_limit(&self.limits)
    }

    /// Reset all counters to zero
    pub fn reset(&mut self) {
        self.counters = ExecutionCounters::new();
    }

    /// Get the remaining steps before limit is reached
    pub fn remaining_steps(&self) -> u64 {
        if self.counters.step_count >= self.limits.max_steps {
            0
        } else {
            self.limits.max_steps - self.counters.step_count
        }
    }

    /// Get the remaining rule invocations before limit is reached
    pub fn remaining_rule_invocations(&self) -> u64 {
        if self.counters.rule_invocation_count >= self.limits.max_rule_invocations {
            0
        } else {
            self.limits.max_rule_invocations - self.counters.rule_invocation_count
        }
    }

    /// Get the remaining loop iterations before limit is reached
    pub fn remaining_loop_iterations(&self) -> u64 {
        if self.counters.loop_iteration_count >= self.limits.max_loop_iterations {
            0
        } else {
            self.limits.max_loop_iterations - self.counters.loop_iteration_count
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_execution_limits_creation() {
        let limits = ExecutionLimits::new(1000, 500, 200);
        assert_eq!(limits.max_steps, 1000);
        assert_eq!(limits.max_rule_invocations, 500);
        assert_eq!(limits.max_loop_iterations, 200);
    }

    #[test]
    fn test_execution_counters() {
        let limits = ExecutionLimits::new(10, 5, 3);
        let mut counters = ExecutionCounters::new();

        // Test step counter
        for i in 1..=10 {
            if i <= 10 {
                assert!(counters.increment_step(&limits).is_ok());
            }
        }
        assert_eq!(counters.step_count, 10);
        assert!(counters.increment_step(&limits).is_err());

        // Test rule invocation counter
        for i in 1..=5 {
            if i <= 5 {
                assert!(counters.increment_rule_invocation(&limits).is_ok());
            }
        }
        assert_eq!(counters.rule_invocation_count, 5);
        assert!(counters.increment_rule_invocation(&limits).is_err());

        // Test loop iteration counter
        for i in 1..=3 {
            if i <= 3 {
                assert!(counters.increment_loop_iteration(&limits).is_ok());
            }
        }
        assert_eq!(counters.loop_iteration_count, 3);
        assert!(counters.increment_loop_iteration(&limits).is_err());
    }

    #[test]
    fn test_step_limiter() {
        let limits = ExecutionLimits::new(5, 3, 2);
        let mut limiter = StepLimiter::new(limits);

        // Test step limiting
        for _ in 0..5 {
            assert!(limiter.increment_step().is_ok());
        }
        assert_eq!(limiter.remaining_steps(), 0);
        assert!(limiter.increment_step().is_err());

        // Reset and test again
        limiter.reset();
        assert_eq!(limiter.counters.step_count, 0);
        assert!(limiter.increment_step().is_ok());
    }

    #[test]
    fn test_exceeds_limit() {
        let limits = ExecutionLimits::new(5, 3, 2);
        let mut counters = ExecutionCounters::new();

        // Increment to exceed step limit
        for _ in 0..6 {
            counters.step_count += 1;
        }

        assert_eq!(counters.exceeds_limit(&limits), Some(StepLimitError::StepLimitExceeded));
    }
}