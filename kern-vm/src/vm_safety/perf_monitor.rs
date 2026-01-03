//! Performance Monitoring Implementation for KERN VM
//! 
//! Implements the performance monitoring system as specified in the safety layer.

use std::collections::HashMap;
use kern_bytecode::Opcode;

/// Performance metrics collected during VM execution
#[derive(Debug, Clone)]
pub struct PerformanceMetrics {
    pub instruction_count: u64,
    pub per_opcode_count: HashMap<Opcode, u64>,
    pub max_stack_depth: u64,
    pub heap_peak_usage: u64,
    pub rule_invocation_counts: HashMap<String, u64>,
    pub graph_node_count: u64,
    pub current_stack_depth: u64,
    pub current_heap_usage: u64,
}

impl PerformanceMetrics {
    pub fn new() -> Self {
        PerformanceMetrics {
            instruction_count: 0,
            per_opcode_count: HashMap::new(),
            max_stack_depth: 0,
            heap_peak_usage: 0,
            rule_invocation_counts: HashMap::new(),
            graph_node_count: 0,
            current_stack_depth: 0,
            current_heap_usage: 0,
        }
    }

    /// Record the execution of an instruction
    pub fn record_instruction(&mut self, opcode: Opcode) {
        self.instruction_count += 1;
        *self.per_opcode_count.entry(opcode).or_insert(0) += 1;
    }

    /// Update stack depth metrics
    pub fn update_stack_depth(&mut self, depth: u64) {
        self.current_stack_depth = depth;
        if depth > self.max_stack_depth {
            self.max_stack_depth = depth;
        }
    }

    /// Update heap usage metrics
    pub fn update_heap_usage(&mut self, usage: u64) {
        self.current_heap_usage = usage;
        if usage > self.heap_peak_usage {
            self.heap_peak_usage = usage;
        }
    }

    /// Record a rule invocation
    pub fn record_rule_invocation(&mut self, rule_name: &str) {
        *self.rule_invocation_counts.entry(rule_name.to_string()).or_insert(0) += 1;
    }

    /// Update graph node count
    pub fn update_graph_node_count(&mut self, count: u64) {
        self.graph_node_count = count;
    }

    /// Reset all metrics to zero
    pub fn reset(&mut self) {
        self.instruction_count = 0;
        self.per_opcode_count.clear();
        self.max_stack_depth = 0;
        self.heap_peak_usage = 0;
        self.rule_invocation_counts.clear();
        self.graph_node_count = 0;
        self.current_stack_depth = 0;
        self.current_heap_usage = 0;
    }

    /// Get the count of a specific opcode execution
    pub fn get_opcode_count(&self, opcode: Opcode) -> u64 {
        *self.per_opcode_count.get(&opcode).unwrap_or(&0)
    }

    /// Get the count of a specific rule invocation
    pub fn get_rule_invocation_count(&self, rule_name: &str) -> u64 {
        *self.rule_invocation_counts.get(rule_name).unwrap_or(&0)
    }
}

/// Performance monitoring configuration
#[derive(Debug, Clone)]
pub struct PerformanceConfig {
    pub enable_instruction_counting: bool,
    pub enable_opcode_breakdown: bool,
    pub enable_stack_monitoring: bool,
    pub enable_heap_monitoring: bool,
    pub enable_rule_monitoring: bool,
    pub enable_graph_monitoring: bool,
}

impl PerformanceConfig {
    pub fn new() -> Self {
        PerformanceConfig {
            enable_instruction_counting: true,
            enable_opcode_breakdown: true,
            enable_stack_monitoring: true,
            enable_heap_monitoring: true,
            enable_rule_monitoring: true,
            enable_graph_monitoring: true,
        }
    }

    /// Enable all performance monitoring features
    pub fn enable_all(&mut self) {
        self.enable_instruction_counting = true;
        self.enable_opcode_breakdown = true;
        self.enable_stack_monitoring = true;
        self.enable_heap_monitoring = true;
        self.enable_rule_monitoring = true;
        self.enable_graph_monitoring = true;
    }

    /// Disable all performance monitoring features
    pub fn disable_all(&mut self) {
        self.enable_instruction_counting = false;
        self.enable_opcode_breakdown = false;
        self.enable_stack_monitoring = false;
        self.enable_heap_monitoring = false;
        self.enable_rule_monitoring = false;
        self.enable_graph_monitoring = false;
    }
}

/// Performance monitor that combines metrics and configuration
#[derive(Debug)]
pub struct PerformanceMonitor {
    pub metrics: PerformanceMetrics,
    pub config: PerformanceConfig,
}

impl PerformanceMonitor {
    pub fn new() -> Self {
        PerformanceMonitor {
            metrics: PerformanceMetrics::new(),
            config: PerformanceConfig::new(),
        }
    }

    /// Record the execution of an instruction if monitoring is enabled
    pub fn record_instruction(&mut self, opcode: Opcode) {
        if self.config.enable_instruction_counting {
            self.metrics.record_instruction(opcode);
        }
    }

    /// Update stack depth metrics if monitoring is enabled
    pub fn update_stack_depth(&mut self, depth: u64) {
        if self.config.enable_stack_monitoring {
            self.metrics.update_stack_depth(depth);
        }
    }

    /// Update heap usage metrics if monitoring is enabled
    pub fn update_heap_usage(&mut self, usage: u64) {
        if self.config.enable_heap_monitoring {
            self.metrics.update_heap_usage(usage);
        }
    }

    /// Record a rule invocation if monitoring is enabled
    pub fn record_rule_invocation(&mut self, rule_name: &str) {
        if self.config.enable_rule_monitoring {
            self.metrics.record_rule_invocation(rule_name);
        }
    }

    /// Update graph node count if monitoring is enabled
    pub fn update_graph_node_count(&mut self, count: u64) {
        if self.config.enable_graph_monitoring {
            self.metrics.update_graph_node_count(count);
        }
    }

    /// Get a snapshot of current performance metrics
    pub fn get_snapshot(&self) -> PerformanceMetrics {
        self.metrics.clone()
    }

    /// Reset all performance metrics
    pub fn reset(&mut self) {
        self.metrics.reset();
    }

    /// Enable or disable instruction counting
    pub fn set_instruction_counting(&mut self, enabled: bool) {
        self.config.enable_instruction_counting = enabled;
    }

    /// Enable or disable opcode breakdown
    pub fn set_opcode_breakdown(&mut self, enabled: bool) {
        self.config.enable_opcode_breakdown = enabled;
    }

    /// Enable or disable stack monitoring
    pub fn set_stack_monitoring(&mut self, enabled: bool) {
        self.config.enable_stack_monitoring = enabled;
    }

    /// Enable or disable heap monitoring
    pub fn set_heap_monitoring(&mut self, enabled: bool) {
        self.config.enable_heap_monitoring = enabled;
    }

    /// Enable or disable rule monitoring
    pub fn set_rule_monitoring(&mut self, enabled: bool) {
        self.config.enable_rule_monitoring = enabled;
    }

    /// Enable or disable graph monitoring
    pub fn set_graph_monitoring(&mut self, enabled: bool) {
        self.config.enable_graph_monitoring = enabled;
    }

    /// Generate a performance report as a string
    pub fn generate_report(&self) -> String {
        let mut report = String::new();
        report.push_str(&format!("Performance Report:\n"));
        report.push_str(&format!("  Total Instructions: {}\n", self.metrics.instruction_count));
        report.push_str(&format!("  Max Stack Depth: {}\n", self.metrics.max_stack_depth));
        report.push_str(&format!("  Heap Peak Usage: {}\n", self.metrics.heap_peak_usage));
        report.push_str(&format!("  Graph Node Count: {}\n", self.metrics.graph_node_count));
        
        if !self.metrics.per_opcode_count.is_empty() {
            report.push_str("  Opcode Breakdown:\n");
            for (opcode, count) in &self.metrics.per_opcode_count {
                report.push_str(&format!("    {:?}: {}\n", opcode, count));
            }
        }
        
        if !self.metrics.rule_invocation_counts.is_empty() {
            report.push_str("  Rule Invocations:\n");
            for (rule_name, count) in &self.metrics.rule_invocation_counts {
                report.push_str(&format!("    {}: {}\n", rule_name, count));
            }
        }
        
        report
    }
}

/// Performance monitoring trait that can be implemented by VM components
pub trait PerformanceTracked {
    /// Record performance metrics for this component
    fn record_performance(&self, monitor: &mut PerformanceMonitor);
}

#[cfg(test)]
mod tests {
    use super::*;
    use kern_bytecode::Opcode;

    #[test]
    fn test_performance_metrics() {
        let mut metrics = PerformanceMetrics::new();
        
        // Test instruction recording
        metrics.record_instruction(Opcode::Nop);
        metrics.record_instruction(Opcode::LoadNum);
        assert_eq!(metrics.instruction_count, 2);
        assert_eq!(metrics.get_opcode_count(Opcode::Nop), 1);
        assert_eq!(metrics.get_opcode_count(Opcode::LoadNum), 1);
        
        // Test stack depth monitoring
        metrics.update_stack_depth(5);
        assert_eq!(metrics.current_stack_depth, 5);
        assert_eq!(metrics.max_stack_depth, 5);
        
        metrics.update_stack_depth(3); // Should not update max
        assert_eq!(metrics.current_stack_depth, 3);
        assert_eq!(metrics.max_stack_depth, 5);
        
        metrics.update_stack_depth(10); // Should update max
        assert_eq!(metrics.current_stack_depth, 10);
        assert_eq!(metrics.max_stack_depth, 10);
        
        // Test heap usage monitoring
        metrics.update_heap_usage(100);
        assert_eq!(metrics.current_heap_usage, 100);
        assert_eq!(metrics.heap_peak_usage, 100);
        
        metrics.update_heap_usage(50); // Should not update peak
        assert_eq!(metrics.current_heap_usage, 50);
        assert_eq!(metrics.heap_peak_usage, 100);
        
        metrics.update_heap_usage(200); // Should update peak
        assert_eq!(metrics.current_heap_usage, 200);
        assert_eq!(metrics.heap_peak_usage, 200);
        
        // Test rule invocation recording
        metrics.record_rule_invocation("test_rule");
        metrics.record_rule_invocation("test_rule");
        metrics.record_rule_invocation("another_rule");
        assert_eq!(metrics.get_rule_invocation_count("test_rule"), 2);
        assert_eq!(metrics.get_rule_invocation_count("another_rule"), 1);
        
        // Test graph node count
        metrics.update_graph_node_count(42);
        assert_eq!(metrics.graph_node_count, 42);
    }

    #[test]
    fn test_performance_monitor() {
        let mut monitor = PerformanceMonitor::new();
        
        // Test instruction recording
        monitor.record_instruction(Opcode::Nop);
        monitor.record_instruction(Opcode::LoadNum);
        assert_eq!(monitor.metrics.instruction_count, 2);
        
        // Test stack depth monitoring
        monitor.update_stack_depth(10);
        assert_eq!(monitor.metrics.max_stack_depth, 10);
        
        // Test heap usage monitoring
        monitor.update_heap_usage(100);
        assert_eq!(monitor.metrics.heap_peak_usage, 100);
        
        // Test rule invocation recording
        monitor.record_rule_invocation("test_rule");
        assert_eq!(monitor.metrics.get_rule_invocation_count("test_rule"), 1);
        
        // Test graph node count
        monitor.update_graph_node_count(5);
        assert_eq!(monitor.metrics.graph_node_count, 5);
        
        // Test disabling monitoring
        monitor.set_instruction_counting(false);
        let prev_count = monitor.metrics.instruction_count;
        monitor.record_instruction(Opcode::Halt);
        assert_eq!(monitor.metrics.instruction_count, prev_count); // Should not change
    }

    #[test]
    fn test_performance_config() {
        let mut config = PerformanceConfig::new();
        
        // Test default state
        assert!(config.enable_instruction_counting);
        assert!(config.enable_opcode_breakdown);
        
        // Test disabling all
        config.disable_all();
        assert!(!config.enable_instruction_counting);
        assert!(!config.enable_opcode_breakdown);
        
        // Test enabling all
        config.enable_all();
        assert!(config.enable_instruction_counting);
        assert!(config.enable_opcode_breakdown);
    }

    #[test]
    fn test_performance_report() {
        let mut monitor = PerformanceMonitor::new();
        
        // Add some metrics
        monitor.record_instruction(Opcode::Nop);
        monitor.record_instruction(Opcode::LoadNum);
        monitor.update_stack_depth(5);
        monitor.update_heap_usage(100);
        monitor.record_rule_invocation("test_rule");
        monitor.update_graph_node_count(3);
        
        let report = monitor.generate_report();
        assert!(report.contains("Total Instructions: 2"));
        assert!(report.contains("Max Stack Depth: 5"));
        assert!(report.contains("Heap Peak Usage: 100"));
        assert!(report.contains("Graph Node Count: 3"));
        assert!(report.contains("test_rule"));
    }
}