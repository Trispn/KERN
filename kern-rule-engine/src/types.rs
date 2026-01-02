use kern_parser::Comparator;
use std::collections::HashMap;
use std::fmt;

// Define the rule engine execution context
#[derive(Debug, Clone)]
pub struct ExecutionContext {
    pub registers: Vec<Option<Value>>, // R0-R15, using Option for uninitialized values
    pub variables: HashMap<String, Value>,
    pub facts: HashMap<String, Value>,
    pub rule_results: HashMap<String, bool>,
    pub current_node_id: Option<u32>,
}

impl ExecutionContext {
    pub fn new() -> Self {
        ExecutionContext {
            registers: (0..16).map(|_| None).collect(), // Initialize with 16 registers (R0-R15)
            variables: HashMap::new(),
            facts: HashMap::new(),
            rule_results: HashMap::new(),
            current_node_id: None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct RuleMatch {
    pub rule_id: u32,
    pub bindings: HashMap<String, Value>,
}

#[derive(Debug, Clone)]
pub struct ConflictEntry {
    pub target_symbol_id: u32,
    pub conflicting_rules: Vec<u32>,
    pub resolution_mode: ResolutionMode,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ResolutionMode {
    Ignore,
    Override,
    Merge,
    Error,
}

// Pattern matching structures
#[derive(Debug, Clone)]
pub enum Pattern {
    Value(Value),
    Variable(String),                // A variable that can match any value
    Composite(String, Vec<Pattern>), // A composite pattern like (entity.field value)
}

#[derive(Debug, Clone)]
pub struct PatternMatch {
    pub bindings: HashMap<String, Value>, // Variable bindings from the match
    pub matched_node: u32,                // The node that matched
}

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Sym(String),
    Num(i64),
    Bool(bool),
    Vec(Vec<Value>),
    Ref(String), // External reference
}

#[derive(Debug)]
pub enum RuleEngineError {
    InvalidNodeType,
    MissingRegisterValue(u16),
    InvalidComparison(Comparator, Value, Value),
    InvalidPredicate(String),
    ExecutionLimitExceeded,
}

#[derive(Debug, Clone, PartialEq)]
pub struct RulePriority {
    pub rule_id: u32,
    pub priority: u32,         // Higher number means higher priority
    pub specificity: u32,      // More specific rules have higher priority
    pub recency: u32,          // More recently added facts might affect priority
    pub activation_count: u32, // How many times the rule has been activated
    pub conflict_score: u32,   // Score based on conflicts with other rules
}

// Define a custom trait for cloning boxed functions
pub trait CloneableFn: Fn(&RulePriority) -> u32 {
    fn clone_box(&self) -> Box<dyn CloneableFn>;
}

impl<T> CloneableFn for T
where
    T: Fn(&RulePriority) -> u32 + Clone + 'static,
{
    fn clone_box(&self) -> Box<dyn CloneableFn> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn CloneableFn> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}

impl fmt::Debug for Box<dyn CloneableFn> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Custom function")
    }
}

impl PartialEq for Box<dyn CloneableFn> {
    fn eq(&self, _other: &Self) -> bool {
        // Two function pointers are equal if they point to the same function
        // For our purposes, we'll consider all custom functions as non-equal
        std::ptr::eq(self.as_ref(), _other.as_ref())
    }
}

#[derive(Clone)]
pub enum PriorityStrategy {
    /// Standard priority based on explicit settings
    Standard,
    /// Priority based on rule specificity (more specific rules fire first)
    SpecificityFirst,
    /// Priority based on recency (newer facts/rules have higher priority)
    RecencyBased,
    /// Priority based on how frequently the rule has been activated
    FrequencyBased,
    /// Priority based on conflict resolution needs
    ConflictResolution,
    /// Custom priority function
    Custom(Box<dyn CloneableFn>),
}

impl fmt::Debug for PriorityStrategy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PriorityStrategy::Standard => write!(f, "Standard"),
            PriorityStrategy::SpecificityFirst => write!(f, "SpecificityFirst"),
            PriorityStrategy::RecencyBased => write!(f, "RecencyBased"),
            PriorityStrategy::FrequencyBased => write!(f, "FrequencyBased"),
            PriorityStrategy::ConflictResolution => write!(f, "ConflictResolution"),
            PriorityStrategy::Custom(_) => write!(f, "Custom"),
        }
    }
}

impl PartialEq for PriorityStrategy {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (PriorityStrategy::Standard, PriorityStrategy::Standard) => true,
            (PriorityStrategy::SpecificityFirst, PriorityStrategy::SpecificityFirst) => true,
            (PriorityStrategy::RecencyBased, PriorityStrategy::RecencyBased) => true,
            (PriorityStrategy::FrequencyBased, PriorityStrategy::FrequencyBased) => true,
            (PriorityStrategy::ConflictResolution, PriorityStrategy::ConflictResolution) => true,
            (PriorityStrategy::Custom(_), PriorityStrategy::Custom(_)) => true, // Consider all custom functions as equal
            _ => false,
        }
    }
}

// Rule execution metadata
#[derive(Debug, Clone)]
pub struct RuleExecutionInfo {
    pub rule_id: u32,
    pub priority: u16,
    pub condition_graph_id: Option<u32>,
    pub action_graph_id: Option<u32>,
    pub dependencies: Vec<u32>,
    pub recursion_limit: u32,
    pub execution_count: u32, // runtime tracker
}

impl RuleExecutionInfo {
    pub fn new(rule_id: u32) -> Self {
        RuleExecutionInfo {
            rule_id,
            priority: 10, // Default normal priority
            condition_graph_id: None,
            action_graph_id: None,
            dependencies: Vec::new(),
            recursion_limit: 10, // Default recursion limit
            execution_count: 0,
        }
    }
}
