//! VM Safety Module
//! 
//! This module implements the safety, limits, and observability layer for the KERN VM
//! as specified in the canonical specification.

pub mod memory_limits;
pub mod step_limits;
pub mod sandbox;
pub mod security;
pub mod perf_monitor;
pub mod limit_errors;