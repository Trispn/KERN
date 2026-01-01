use crate::flow_execution_context::FlowExecutionContext;
use crate::types::{SymbolTable, Value};
use std::collections::HashMap;

/// Manages flow execution contexts
pub struct ContextManager {
    contexts: HashMap<u32, FlowExecutionContext>,
    current_context_id: u32,
}

impl ContextManager {
    pub fn new() -> Self {
        let mut contexts = HashMap::new();
        let initial_context = FlowExecutionContext::new(0); // Initial context with ID 0
        contexts.insert(0, initial_context);

        ContextManager {
            contexts,
            current_context_id: 0,
        }
    }

    /// Creates a new execution context
    pub fn create_context(&mut self, flow_id: u32) -> u32 {
        let context_id = self.contexts.len() as u32;
        let new_context = FlowExecutionContext::new(flow_id);
        self.contexts.insert(context_id, new_context);
        context_id
    }

    /// Creates a new execution context with a parent
    pub fn create_context_with_parent(&mut self, flow_id: u32, parent_id: u32) -> Result<u32, ContextError> {
        let parent_context = self.get_context(parent_id)?.clone();
        let context_id = self.contexts.len() as u32;
        let new_context = FlowExecutionContext::with_parent(flow_id, parent_context);
        self.contexts.insert(context_id, new_context);
        Ok(context_id)
    }

    /// Gets a mutable reference to the current context
    pub fn get_current_context_mut(&mut self) -> Result<&mut FlowExecutionContext, ContextError> {
        self.get_context_mut(self.current_context_id)
    }

    /// Gets a reference to the current context
    pub fn get_current_context(&self) -> Result<&FlowExecutionContext, ContextError> {
        self.get_context(self.current_context_id)
    }

    /// Gets a mutable reference to a specific context
    pub fn get_context_mut(&mut self, context_id: u32) -> Result<&mut FlowExecutionContext, ContextError> {
        self.contexts.get_mut(&context_id)
            .ok_or(ContextError::ContextNotFound(context_id))
    }

    /// Gets a reference to a specific context
    pub fn get_context(&self, context_id: u32) -> Result<&FlowExecutionContext, ContextError> {
        self.contexts.get(&context_id)
            .ok_or(ContextError::ContextNotFound(context_id))
    }

    /// Sets the current context
    pub fn set_current_context(&mut self, context_id: u32) -> Result<(), ContextError> {
        if self.contexts.contains_key(&context_id) {
            self.current_context_id = context_id;
            Ok(())
        } else {
            Err(ContextError::ContextNotFound(context_id))
        }
    }

    /// Gets the ID of the current context
    pub fn get_current_context_id(&self) -> u32 {
        self.current_context_id
    }

    /// Clones a context
    pub fn clone_context(&mut self, source_id: u32) -> Result<u32, ContextError> {
        let source_context = self.get_context(source_id)?.clone();
        let new_context_id = self.contexts.len() as u32;
        self.contexts.insert(new_context_id, source_context);
        Ok(new_context_id)
    }

    /// Merges symbols from a child context into a parent context
    pub fn merge_context(&mut self, child_id: u32, parent_id: u32) -> Result<(), ContextError> {
        let child_context = self.get_context(child_id)?.clone();
        let parent_context = self.get_context_mut(parent_id)?;

        // Merge local symbols from child to parent
        for (key, value) in &child_context.local_symbols.symbols {
            parent_context.local_symbols.insert(key.clone(), value.clone());
        }

        Ok(())
    }

    /// Removes a context
    pub fn remove_context(&mut self, context_id: u32) -> Result<(), ContextError> {
        if context_id == 0 {
            // Don't allow removal of the initial context
            return Err(ContextError::CannotRemoveInitialContext);
        }

        if self.contexts.remove(&context_id).is_some() {
            // If we're removing the current context, switch to parent if available
            if context_id == self.current_context_id {
                if let Some(_context) = self.contexts.get(&0) { // Fallback to initial context
                    self.current_context_id = 0;
                }
            }
            Ok(())
        } else {
            Err(ContextError::ContextNotFound(context_id))
        }
    }
}

#[derive(Debug)]
pub enum ContextError {
    ContextNotFound(u32),
    CannotRemoveInitialContext,
}