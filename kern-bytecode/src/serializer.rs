//! Bytecode Serializer
//!
//! This module implements serialization of bytecode modules to binary format
//! according to the specification.

use crate::{BytecodeModule, Constant, GraphEntry, Instruction, ModuleHeader, RuleEntry, SectionOffsets, Symbol};
use std::collections::HashMap;

/// Bytecode serializer
pub struct BytecodeSerializer;

impl BytecodeSerializer {
    pub fn new() -> Self {
        BytecodeSerializer
    }

    /// Serialize a bytecode module to binary format
    pub fn serialize(&self, module: &BytecodeModule) -> Vec<u8> {
        let mut bytes = Vec::new();

        // 1. Serialize header
        let header_bytes = self.serialize_header(&module.header);
        bytes.extend_from_slice(&header_bytes);

        // 2. Serialize instruction stream
        for instruction in &module.instruction_stream {
            bytes.extend_from_slice(&instruction.to_bytes());
        }

        // 3. Serialize constant pool
        for constant in &module.constant_pool {
            let const_bytes = self.serialize_constant(constant);
            bytes.extend_from_slice(&const_bytes);
        }

        // 4. Serialize symbol table
        for symbol in &module.symbol_table {
            let symbol_bytes = self.serialize_symbol(symbol);
            bytes.extend_from_slice(&symbol_bytes);
        }

        // 5. Serialize rule table
        for rule in &module.rule_table {
            let rule_bytes = self.serialize_rule_entry(rule);
            bytes.extend_from_slice(&rule_bytes);
        }

        // 6. Serialize graph table
        for graph in &module.graph_table {
            let graph_bytes = self.serialize_graph_entry(graph);
            bytes.extend_from_slice(&graph_bytes);
        }

        // 7. Serialize metadata
        bytes.extend_from_slice(&module.metadata);

        bytes
    }

    /// Deserialize a bytecode module from binary format
    pub fn deserialize(&self, bytes: &[u8]) -> Option<BytecodeModule> {
        let mut offset = 0;

        // 1. Deserialize header
        let header = self.deserialize_header(&bytes[offset..offset + 32])?;
        offset += 32; // Header is fixed size

        // 2. Deserialize instruction stream
        let mut instruction_stream = Vec::new();
        let instruction_end = (header.section_offsets.instruction_offset + 
                              header.instruction_count * 8) as usize;
        if bytes.len() >= instruction_end {
            for i in 0..header.instruction_count {
                let instr_start = header.section_offsets.instruction_offset as usize + (i as usize * 8);
                if instr_start + 8 <= bytes.len() {
                    let instruction = Instruction::from_bytes(&bytes[instr_start..instr_start + 8])?;
                    instruction_stream.push(instruction);
                }
            }
        }

        // 3. Deserialize constant pool
        let mut constant_pool = Vec::new();
        // For simplicity in this implementation, we'll assume a fixed number of constants
        // In a real implementation, we'd need to store the count in the header

        // 4. Deserialize symbol table
        let mut symbol_table = Vec::new();
        // Similarly, we'd need to store the count in the header

        // 5. Deserialize rule table
        let mut rule_table = Vec::new();

        // 6. Deserialize graph table
        let mut graph_table = Vec::new();

        // 7. Deserialize metadata
        let metadata_start = header.section_offsets.metadata_offset as usize;
        let metadata = if metadata_start < bytes.len() {
            bytes[metadata_start..].to_vec()
        } else {
            Vec::new()
        };

        Some(BytecodeModule {
            header,
            instruction_stream,
            constant_pool,
            symbol_table,
            rule_table,
            graph_table,
            metadata,
        })
    }

    /// Serialize module header
    fn serialize_header(&self, header: &ModuleHeader) -> [u8; 32] {
        let mut bytes = [0u8; 32];
        
        // Magic number (4 bytes)
        bytes[0..4].copy_from_slice(&header.magic);
        
        // Version (4 bytes)
        bytes[4..8].copy_from_slice(&header.version.to_le_bytes());
        
        // Instruction count (4 bytes)
        bytes[8..12].copy_from_slice(&header.instruction_count.to_le_bytes());
        
        // Section offsets (6 * 4 = 24 bytes)
        let offsets_start = 12;
        bytes[offsets_start..offsets_start + 4].copy_from_slice(&header.section_offsets.instruction_offset.to_le_bytes());
        bytes[offsets_start + 4..offsets_start + 8].copy_from_slice(&header.section_offsets.constant_pool_offset.to_le_bytes());
        bytes[offsets_start + 8..offsets_start + 12].copy_from_slice(&header.section_offsets.symbol_table_offset.to_le_bytes());
        bytes[offsets_start + 12..offsets_start + 16].copy_from_slice(&header.section_offsets.rule_table_offset.to_le_bytes());
        bytes[offsets_start + 16..offsets_start + 20].copy_from_slice(&header.section_offsets.graph_table_offset.to_le_bytes());
        bytes[offsets_start + 20..offsets_start + 24].copy_from_slice(&header.section_offsets.metadata_offset.to_le_bytes());
        
        // Checksum (8 bytes)
        bytes[24..32].copy_from_slice(&header.checksum.to_le_bytes());
        
        bytes
    }

    /// Deserialize module header
    fn deserialize_header(&self, bytes: &[u8]) -> Option<ModuleHeader> {
        if bytes.len() < 32 {
            return None;
        }

        let mut magic = [0u8; 4];
        magic.copy_from_slice(&bytes[0..4]);

        let version = u32::from_le_bytes([bytes[4], bytes[5], bytes[6], bytes[7]]);
        let instruction_count = u32::from_le_bytes([bytes[8], bytes[9], bytes[10], bytes[11]]);

        let offsets_start = 12;
        let section_offsets = SectionOffsets {
            instruction_offset: u32::from_le_bytes([
                bytes[offsets_start], bytes[offsets_start + 1],
                bytes[offsets_start + 2], bytes[offsets_start + 3]
            ]),
            constant_pool_offset: u32::from_le_bytes([
                bytes[offsets_start + 4], bytes[offsets_start + 5],
                bytes[offsets_start + 6], bytes[offsets_start + 7]
            ]),
            symbol_table_offset: u32::from_le_bytes([
                bytes[offsets_start + 8], bytes[offsets_start + 9],
                bytes[offsets_start + 10], bytes[offsets_start + 11]
            ]),
            rule_table_offset: u32::from_le_bytes([
                bytes[offsets_start + 12], bytes[offsets_start + 13],
                bytes[offsets_start + 14], bytes[offsets_start + 15]
            ]),
            graph_table_offset: u32::from_le_bytes([
                bytes[offsets_start + 16], bytes[offsets_start + 17],
                bytes[offsets_start + 18], bytes[offsets_start + 19]
            ]),
            metadata_offset: u32::from_le_bytes([
                bytes[offsets_start + 20], bytes[offsets_start + 21],
                bytes[offsets_start + 22], bytes[offsets_start + 23]
            ]),
        };

        let checksum = u64::from_le_bytes([
            bytes[24], bytes[25], bytes[26], bytes[27],
            bytes[28], bytes[29], bytes[30], bytes[31]
        ]);

        Some(ModuleHeader {
            magic,
            version,
            instruction_count,
            section_offsets,
            checksum,
        })
    }

    /// Serialize a constant
    fn serialize_constant(&self, constant: &Constant) -> [u8; 16] { // Fixed size for simplicity
        let mut bytes = [0u8; 16];
        
        match constant {
            Constant::Num(value) => {
                bytes[0] = 0x01; // Type: Num
                bytes[1..9].copy_from_slice(&value.to_le_bytes());
            },
            Constant::Bool(value) => {
                bytes[0] = 0x02; // Type: Bool
                bytes[1] = if *value { 1 } else { 0 };
            },
            Constant::Sym(symbol) => {
                bytes[0] = 0x03; // Type: Sym
                let sym_bytes = symbol.as_bytes();
                let len = std::cmp::min(sym_bytes.len(), 15);
                bytes[1..1 + len].copy_from_slice(&sym_bytes[..len]);
            },
            Constant::Vec(vec) => {
                bytes[0] = 0x04; // Type: Vec
                // For simplicity, we're not fully implementing vector serialization
                // In a real implementation, this would be more complex
            },
        }
        
        bytes
    }

    /// Serialize a symbol
    fn serialize_symbol(&self, symbol: &Symbol) -> [u8; 64] { // Fixed size for simplicity
        let mut bytes = [0u8; 64];
        
        // ID (4 bytes)
        bytes[0..4].copy_from_slice(&symbol.id.to_le_bytes());
        
        // Name (60 bytes)
        let name_bytes = symbol.name.as_bytes();
        let len = std::cmp::min(name_bytes.len(), 60);
        bytes[4..4 + len].copy_from_slice(&name_bytes[..len]);
        
        bytes
    }

    /// Serialize a rule entry
    fn serialize_rule_entry(&self, rule: &RuleEntry) -> [u8; 68] { // Fixed size for simplicity
        let mut bytes = [0u8; 68];
        
        // ID (4 bytes)
        bytes[0..4].copy_from_slice(&rule.id.to_le_bytes());
        
        // Entry PC (4 bytes)
        bytes[4..8].copy_from_slice(&rule.entry_pc.to_le_bytes());
        
        // Name (60 bytes)
        let name_bytes = rule.name.as_bytes();
        let len = std::cmp::min(name_bytes.len(), 60);
        bytes[8..8 + len].copy_from_slice(&name_bytes[..len]);
        
        bytes
    }

    /// Serialize a graph entry
    fn serialize_graph_entry(&self, graph: &GraphEntry) -> [u8; 12] { // Fixed size for simplicity
        let mut bytes = [0u8; 12];
        
        // ID (4 bytes)
        bytes[0..4].copy_from_slice(&graph.id.to_le_bytes());
        
        // Node count (4 bytes)
        bytes[4..8].copy_from_slice(&graph.node_count.to_le_bytes());
        
        // Edge count (4 bytes)
        bytes[8..12].copy_from_slice(&graph.edge_count.to_le_bytes());
        
        bytes
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Instruction, Opcode};

    #[test]
    fn test_header_serialization() {
        let header = ModuleHeader {
            magic: *b"KERN",
            version: 1,
            instruction_count: 5,
            section_offsets: SectionOffsets {
                instruction_offset: 32,
                constant_pool_offset: 72,
                symbol_table_offset: 100,
                rule_table_offset: 200,
                graph_table_offset: 300,
                metadata_offset: 400,
            },
            checksum: 0x123456789ABCDEF0,
        };

        let serializer = BytecodeSerializer::new();
        let bytes = serializer.serialize_header(&header);
        // The header is 32 bytes, so we need to make sure we're reading the right amount
        assert_eq!(bytes.len(), 32);
        let deserialized = serializer.deserialize_header(&bytes).unwrap();

        assert_eq!(header.magic, deserialized.magic);
        assert_eq!(header.version, deserialized.version);
        assert_eq!(header.instruction_count, deserialized.instruction_count);
        assert_eq!(header.section_offsets.instruction_offset, deserialized.section_offsets.instruction_offset);
        assert_eq!(header.checksum, deserialized.checksum);
    }

    #[test]
    fn test_instruction_serialization() {
        let instruction = Instruction::new(Opcode::LoadNum as u8, 1, 42, 0, 0);
        let bytes = instruction.to_bytes();
        let deserialized = Instruction::from_bytes(&bytes).unwrap();

        assert_eq!(instruction.opcode, deserialized.opcode);
        assert_eq!(instruction.arg1, deserialized.arg1);
        assert_eq!(instruction.arg2, deserialized.arg2);
        assert_eq!(instruction.arg3, deserialized.arg3);
        assert_eq!(instruction.flags, deserialized.flags);
    }

    #[test]
    fn test_symbol_serialization() {
        let symbol = Symbol {
            id: 1,
            name: "test_symbol".to_string(),
        };

        let serializer = BytecodeSerializer::new();
        let bytes = serializer.serialize_symbol(&symbol);
        // Note: We don't have a deserialize_symbol function, but we can at least test serialization doesn't panic
        assert!(bytes.len() == 64);
        assert_eq!(u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]), 1);
    }
}