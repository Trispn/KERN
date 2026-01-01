// Import the shared types
use crate::types::Value;

/// FlowStepExecutionInfo contains information about a single flow step
#[derive(Debug, Clone)]
pub struct FlowStepExecutionInfo {
    pub step_id: u32,
    pub condition_graph_id: Option<u32>,
    pub action_graph_id: u32,
    pub evaluated: bool,
    pub cached_result: Option<Value>,
}

impl FlowStepExecutionInfo {
    pub fn new(step_id: u32, action_graph_id: u32) -> Self {
        FlowStepExecutionInfo {
            step_id,
            condition_graph_id: None,
            action_graph_id,
            evaluated: false,
            cached_result: None,
        }
    }

    pub fn with_condition(step_id: u32, condition_graph_id: Option<u32>, action_graph_id: u32) -> Self {
        FlowStepExecutionInfo {
            step_id,
            condition_graph_id,
            action_graph_id,
            evaluated: false,
            cached_result: None,
        }
    }

    pub fn mark_evaluated(&mut self, result: Value) {
        self.evaluated = true;
        self.cached_result = Some(result);
    }

    pub fn get_result(&self) -> Option<&Value> {
        self.cached_result.as_ref()
    }
}