use crate::{BytecodeModule, ModuleHeader, SectionOffsets, Opcode, Instruction};
use crate::lir::{LirOp, LirProgram, Register};
use crate::lir_builder::LirBuilder;
use crate::register_allocator::LinearScanAllocator;
use crate::emitter::BytecodeEmitter;
use kern_ast::ProgramNode as Program;
use kern_graph_builder::{ExecutionGraph, SpecializedNode, EdgeType};
use serde::Serialize;
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, Serialize)]
pub struct BytecodeCompiler {
    // Compiler state
}

impl BytecodeCompiler {
    pub fn new() -> Self {
        BytecodeCompiler {}
    }

    pub fn compile(&mut self, _program: &Program) -> Result<BytecodeModule, String> {
        // Placeholder implementation for AST -> Bytecode
        Ok(BytecodeModule {
            header: ModuleHeader {
                magic: *b"KERN",
                version: 1,
                instruction_count: 0,
                section_offsets: SectionOffsets {
                    instruction_offset: 0,
                    constant_pool_offset: 0,
                    symbol_table_offset: 0,
                    rule_table_offset: 0,
                    graph_table_offset: 0,
                    metadata_offset: 0,
                },
                checksum: 0,
            },
            instruction_stream: Vec::new(),
            constant_pool: Vec::new(),
            symbol_table: Vec::new(),
            rule_table: Vec::new(),
            graph_table: Vec::new(),
            metadata: Vec::new(),
        })
    }

    pub fn compile_graph(&mut self, graph: &ExecutionGraph) -> BytecodeModule {
        let mut lir_builder = LirBuilder::new();
        let mut node_regs: HashMap<u32, Register> = HashMap::new();
        let mut visited: HashSet<u32> = HashSet::new();
        
        // Build adjacency lists
        let mut adj_data: HashMap<u32, Vec<u32>> = HashMap::new();
        let mut adj_control: HashMap<u32, Vec<u32>> = HashMap::new();
        
        for edge in &graph.edges {
            match edge.edge_type {
                EdgeType::Data => {
                    adj_data.entry(edge.from_node).or_default().push(edge.to_node);
                }
                EdgeType::Control | EdgeType::Condition => {
                    adj_control.entry(edge.from_node).or_default().push(edge.to_node);
                }
            }
        }
        
        // Process entry points
        for entry in &graph.entry_points {
            self.emit_node_recursive(
                entry.node_id, 
                graph, 
                &mut lir_builder, 
                &mut node_regs, 
                &mut visited,
                &adj_data,
                &adj_control
            );
        }
        
        // Allocate registers
        let lir_program = lir_builder.build();
        let mut allocator = LinearScanAllocator::new();
        let allocation = allocator.allocate(&lir_program);
        
        // Emit bytecode
        let mut emitter = BytecodeEmitter::new();
        let instructions = emitter.emit_from_lir(&lir_program.instructions, &allocation);
        
        // Construct module
        BytecodeModule {
            header: ModuleHeader {
                magic: *b"KERN",
                version: 1,
                instruction_count: instructions.len() as u32,
                section_offsets: SectionOffsets {
                    instruction_offset: 32, // Header size
                    constant_pool_offset: 0,
                    symbol_table_offset: 0,
                    rule_table_offset: 0,
                    graph_table_offset: 0,
                    metadata_offset: 0,
                },
                checksum: 0,
            },
            instruction_stream: instructions,
            constant_pool: Vec::new(), // TODO: Extract constants
            symbol_table: Vec::new(),
            rule_table: Vec::new(),
            graph_table: Vec::new(),
            metadata: Vec::new(),
        }
    }
    
    fn emit_node_recursive(
        &self,
        node_id: u32,
        graph: &ExecutionGraph,
        builder: &mut LirBuilder,
        node_regs: &mut HashMap<u32, Register>,
        visited: &mut HashSet<u32>,
        adj_data: &HashMap<u32, Vec<u32>>,
        adj_control: &HashMap<u32, Vec<u32>>,
    ) -> Option<Register> {
        if visited.contains(&node_id) {
            return node_regs.get(&node_id).cloned();
        }
        visited.insert(node_id);
        
        // Find the node
        let node = graph.nodes.iter().find(|n| n.id() == node_id);
        if node.is_none() {
            return None;
        }
        let node = node.unwrap();
        let base_node = node.base();
        
        // Process data dependencies first (children)
        let mut input_regs = Vec::new();
        if let Some(children) = adj_data.get(&node_id) {
            for child_id in children {
                if let Some(reg) = self.emit_node_recursive(*child_id, graph, builder, node_regs, visited, adj_data, adj_control) {
                    input_regs.push(reg);
                }
            }
        }
        
        // Emit instruction for this node
        let output_reg = match node {
             SpecializedNode::Value(val_node) => {
                 match base_node.opcode {
                     0x10 => { // LOAD_SYM
                         Some(builder.load_sym(&val_node.value_sym))
                     },
                     0x11 => { // LOAD_NUM
                         let reg = builder.program.alloc_register();
                         builder.program.add_instruction(crate::lir::LirInstruction {
                             op: LirOp::LoadNum(val_node.value_num as i64),
                             dst: Some(reg),
                             src1: None,
                             src2: None,
                             immediate: Some(val_node.value_num as i64),
                             label: None,
                         });
                         Some(reg)
                     },
                     _ => None
                 }
             },
             SpecializedNode::Io(io_node) => {
                 match base_node.opcode {
                     0x60 => { // EXT_CALL
                         builder.program.add_instruction(crate::lir::LirInstruction {
                             op: LirOp::Call(io_node.name.clone(), input_regs.clone()),
                             dst: None, 
                             src1: None,
                             src2: None,
                             immediate: None,
                             label: None,
                         });
                         None
                     },
                     _ => None
                 }
             },
             _ => match base_node.opcode {
                 0x13 => { // COMPARE
                     if input_regs.len() >= 2 {
                         // Assume Eq for now
                         let reg = builder.program.alloc_register();
                         builder.program.add_instruction(crate::lir::LirInstruction {
                             op: LirOp::CmpEq(input_regs[0], input_regs[1]),
                             dst: Some(reg),
                             src1: Some(input_regs[0]),
                             src2: Some(input_regs[1]),
                             immediate: None,
                             label: None,
                         });
                         Some(reg)
                     } else {
                         None
                     }
                 },
                 0x31 => { // RULE_EVAL
                     // Just a placeholder for rule entry
                     None
                 },
                 _ => None,
             }
        };
        
        if let Some(reg) = output_reg {
            node_regs.insert(node_id, reg);
        }
        
        // Process control flow (next blocks)
        if let Some(next_nodes) = adj_control.get(&node_id) {
            for next_id in next_nodes {
                self.emit_node_recursive(*next_id, graph, builder, node_regs, visited, adj_data, adj_control);
            }
        }
        
        output_reg
    }
}
