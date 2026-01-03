//! Memory Limits Implementation for KERN VM
//! 
//! Implements the memory budget and enforcement system as specified in the safety layer.

use std::collections::HashMap;

/// Memory limits configuration structure
#[derive(Debug, Clone)]
pub struct MemoryLimits {
    pub max_code_bytes: usize,
    pub max_const_bytes: usize,
    pub max_stack_bytes: usize,
    pub max_heap_bytes: usize,
    pub max_meta_bytes: usize,
}

impl MemoryLimits {
    pub fn new(
        max_code_bytes: usize,
        max_const_bytes: usize,
        max_stack_bytes: usize,
        max_heap_bytes: usize,
        max_meta_bytes: usize,
    ) -> Self {
        MemoryLimits {
            max_code_bytes,
            max_const_bytes,
            max_stack_bytes,
            max_heap_bytes,
            max_meta_bytes,
        }
    }

    /// Default memory limits for testing purposes
    pub fn default() -> Self {
        MemoryLimits {
            max_code_bytes: 1024 * 100,      // 100KB
            max_const_bytes: 1024 * 50,      // 50KB
            max_stack_bytes: 1024 * 256,     // 256KB
            max_heap_bytes: 1024 * 1024,     // 1MB
            max_meta_bytes: 1024 * 10,       // 10KB
        }
    }
}

/// Memory usage tracker for runtime enforcement
#[derive(Debug, Clone)]
pub struct MemoryUsage {
    pub code_used: usize,
    pub const_used: usize,
    pub stack_used: usize,
    pub heap_used: usize,
    pub meta_used: usize,
}

impl MemoryUsage {
    pub fn new() -> Self {
        MemoryUsage {
            code_used: 0,
            const_used: 0,
            stack_used: 0,
            heap_used: 0,
            meta_used: 0,
        }
    }

    /// Check if adding the specified amount would exceed limits
    pub fn would_exceed_limit(&self, limits: &MemoryLimits) -> Option<MemoryLimitType> {
        if self.code_used > limits.max_code_bytes {
            return Some(MemoryLimitType::Code);
        }
        if self.const_used > limits.max_const_bytes {
            return Some(MemoryLimitType::Const);
        }
        if self.stack_used > limits.max_stack_bytes {
            return Some(MemoryLimitType::Stack);
        }
        if self.heap_used > limits.max_heap_bytes {
            return Some(MemoryLimitType::Heap);
        }
        if self.meta_used > limits.max_meta_bytes {
            return Some(MemoryLimitType::Meta);
        }
        None
    }

    /// Check if current usage exceeds limits
    pub fn exceeds_limit(&self, limits: &MemoryLimits) -> bool {
        self.would_exceed_limit(limits).is_some()
    }
}

/// Types of memory limits
#[derive(Debug, Clone, PartialEq)]
pub enum MemoryLimitType {
    Code,
    Const,
    Stack,
    Heap,
    Meta,
}

/// Memory region types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MemoryRegion {
    Code,
    Const,
    Stack,
    Heap,
    Meta,
}

/// Memory manager for tracking and enforcing limits
#[derive(Debug)]
pub struct MemoryManager {
    pub limits: MemoryLimits,
    pub usage: MemoryUsage,
    pub region_usage: HashMap<MemoryRegion, usize>,
}

impl MemoryManager {
    pub fn new(limits: MemoryLimits) -> Self {
        let mut region_usage = HashMap::new();
        region_usage.insert(MemoryRegion::Code, 0);
        region_usage.insert(MemoryRegion::Const, 0);
        region_usage.insert(MemoryRegion::Stack, 0);
        region_usage.insert(MemoryRegion::Heap, 0);
        region_usage.insert(MemoryRegion::Meta, 0);

        MemoryManager {
            limits,
            usage: MemoryUsage::new(),
            region_usage,
        }
    }

    /// Allocate memory in the specified region
    pub fn allocate(&mut self, region: MemoryRegion, size: usize) -> Result<(), MemoryLimitError> {
        let current_usage = self.region_usage.get_mut(&region).unwrap();
        let new_usage = current_usage.saturating_add(size);

        // Update the usage based on region type
        match region {
            MemoryRegion::Code => {
                if new_usage > self.limits.max_code_bytes {
                    return Err(MemoryLimitError::CodeLimitExceeded);
                }
                self.usage.code_used = new_usage;
            }
            MemoryRegion::Const => {
                if new_usage > self.limits.max_const_bytes {
                    return Err(MemoryLimitError::ConstLimitExceeded);
                }
                self.usage.const_used = new_usage;
            }
            MemoryRegion::Stack => {
                if new_usage > self.limits.max_stack_bytes {
                    return Err(MemoryLimitError::StackLimitExceeded);
                }
                self.usage.stack_used = new_usage;
            }
            MemoryRegion::Heap => {
                if new_usage > self.limits.max_heap_bytes {
                    return Err(MemoryLimitError::HeapLimitExceeded);
                }
                self.usage.heap_used = new_usage;
            }
            MemoryRegion::Meta => {
                if new_usage > self.limits.max_meta_bytes {
                    return Err(MemoryLimitError::MetaLimitExceeded);
                }
                self.usage.meta_used = new_usage;
            }
        }

        *current_usage = new_usage;
        Ok(())
    }

    /// Deallocate memory in the specified region
    pub fn deallocate(&mut self, region: MemoryRegion, size: usize) {
        let current_usage = self.region_usage.get_mut(&region).unwrap();
        let new_usage = current_usage.saturating_sub(size);

        // Update the usage based on region type
        match region {
            MemoryRegion::Code => self.usage.code_used = new_usage,
            MemoryRegion::Const => self.usage.const_used = new_usage,
            MemoryRegion::Stack => self.usage.stack_used = new_usage,
            MemoryRegion::Heap => self.usage.heap_used = new_usage,
            MemoryRegion::Meta => self.usage.meta_used = new_usage,
        }

        *current_usage = new_usage;
    }

    /// Check if allocation would exceed limits without actually allocating
    pub fn would_allocate_exceed_limit(&self, region: MemoryRegion, size: usize) -> bool {
        let current_usage = self.region_usage[&region];
        let new_usage = current_usage.saturating_add(size);

        match region {
            MemoryRegion::Code => new_usage > self.limits.max_code_bytes,
            MemoryRegion::Const => new_usage > self.limits.max_const_bytes,
            MemoryRegion::Stack => new_usage > self.limits.max_stack_bytes,
            MemoryRegion::Heap => new_usage > self.limits.max_heap_bytes,
            MemoryRegion::Meta => new_usage > self.limits.max_meta_bytes,
        }
    }
}

/// Memory limit errors
#[derive(Debug, Clone, PartialEq)]
pub enum MemoryLimitError {
    CodeLimitExceeded,
    ConstLimitExceeded,
    StackLimitExceeded,
    HeapLimitExceeded,
    MetaLimitExceeded,
}

impl std::fmt::Display for MemoryLimitError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MemoryLimitError::CodeLimitExceeded => write!(f, "Code memory limit exceeded"),
            MemoryLimitError::ConstLimitExceeded => write!(f, "Constant memory limit exceeded"),
            MemoryLimitError::StackLimitExceeded => write!(f, "Stack memory limit exceeded"),
            MemoryLimitError::HeapLimitExceeded => write!(f, "Heap memory limit exceeded"),
            MemoryLimitError::MetaLimitExceeded => write!(f, "Meta memory limit exceeded"),
        }
    }
}

impl std::error::Error for MemoryLimitError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_memory_limits_creation() {
        let limits = MemoryLimits::new(100, 200, 300, 400, 500);
        assert_eq!(limits.max_code_bytes, 100);
        assert_eq!(limits.max_const_bytes, 200);
        assert_eq!(limits.max_stack_bytes, 300);
        assert_eq!(limits.max_heap_bytes, 400);
        assert_eq!(limits.max_meta_bytes, 500);
    }

    #[test]
    fn test_memory_manager_allocation() {
        let limits = MemoryLimits::new(100, 100, 100, 100, 100);
        let mut manager = MemoryManager::new(limits);

        // Test successful allocation
        assert!(manager.allocate(MemoryRegion::Heap, 50).is_ok());
        assert_eq!(manager.usage.heap_used, 50);

        // Test allocation that would exceed limit
        assert!(manager.allocate(MemoryRegion::Heap, 60).is_err());
        assert_eq!(manager.usage.heap_used, 50); // Should remain unchanged

        // Test successful deallocation
        manager.deallocate(MemoryRegion::Heap, 20);
        assert_eq!(manager.usage.heap_used, 30);
    }

    #[test]
    fn test_memory_usage_exceeds_limit() {
        let limits = MemoryLimits::new(100, 100, 100, 100, 100);
        let mut usage = MemoryUsage::new();
        
        usage.heap_used = 150; // Exceeds limit of 100
        assert!(usage.exceeds_limit(&limits));
        assert_eq!(usage.would_exceed_limit(&limits), Some(MemoryLimitType::Heap));
    }
}