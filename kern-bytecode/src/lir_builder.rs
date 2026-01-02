//! Linear IR (LIR) Builder
//!
//! This module provides utilities for building LIR programs programmatically.

use crate::lir::{LirInstruction, LirOp, LirProgram, Register};

pub struct LirBuilder {
    program: LirProgram,
}

impl LirBuilder {
    pub fn new() -> Self {
        LirBuilder {
            program: LirProgram::new(),
        }
    }

    pub fn build(mut self) -> LirProgram {
        self.program
    }

    // Control Flow Operations
    pub fn nop(&mut self) -> &mut Self {
        self.program.add_instruction(LirInstruction {
            op: LirOp::Nop,
            dst: None,
            src1: None,
            src2: None,
            immediate: None,
            label: None,
        });
        self
    }

    pub fn jmp(&mut self, label: u32) -> &mut Self {
        self.program.add_instruction(LirInstruction {
            op: LirOp::Jmp(label),
            dst: None,
            src1: None,
            src2: None,
            immediate: Some(label as i64),
            label: None,
        });
        self
    }

    pub fn jmp_if(&mut self, condition: Register, label: u32) -> &mut Self {
        self.program.add_instruction(LirInstruction {
            op: LirOp::JmpIf(condition, label),
            dst: None,
            src1: Some(condition),
            src2: None,
            immediate: Some(label as i64),
            label: None,
        });
        self
    }

    pub fn jmp_if_not(&mut self, condition: Register, label: u32) -> &mut Self {
        self.program.add_instruction(LirInstruction {
            op: LirOp::JmpIfNot(condition, label),
            dst: None,
            src1: Some(condition),
            src2: None,
            immediate: Some(label as i64),
            label: None,
        });
        self
    }

    pub fn halt(&mut self) -> &mut Self {
        self.program.add_instruction(LirInstruction {
            op: LirOp::Halt,
            dst: None,
            src1: None,
            src2: None,
            immediate: None,
            label: None,
        });
        self
    }

    pub fn label(&mut self, label: u32) -> &mut Self {
        self.program.add_instruction(LirInstruction {
            op: LirOp::Label(label),
            dst: None,
            src1: None,
            src2: None,
            immediate: None,
            label: Some(label),
        });
        self
    }

    // Data & Symbol Operations
    pub fn load_sym(&mut self, symbol: &str) -> Register {
        let dst = self.program.alloc_register();
        self.program.add_instruction(LirInstruction {
            op: LirOp::LoadSym(symbol.to_string()),
            dst: Some(dst),
            src1: None,
            src2: None,
            immediate: None,
            label: None,
        });
        dst
    }

    pub fn load_num(&mut self, value: i64) -> Register {
        let dst = self.program.alloc_register();
        self.program.add_instruction(LirInstruction {
            op: LirOp::LoadNum(value),
            dst: Some(dst),
            src1: None,
            src2: None,
            immediate: Some(value),
            label: None,
        });
        dst
    }

    pub fn load_bool(&mut self, value: bool) -> Register {
        let dst = self.program.alloc_register();
        self.program.add_instruction(LirInstruction {
            op: LirOp::LoadBool(value),
            dst: Some(dst),
            src1: None,
            src2: None,
            immediate: Some(if value { 1 } else { 0 }),
            label: None,
        });
        dst
    }

    pub fn move_reg(&mut self, src: Register, dst: Register) -> &mut Self {
        self.program.add_instruction(LirInstruction {
            op: LirOp::Move(src, dst),
            dst: Some(dst),
            src1: Some(src),
            src2: None,
            immediate: None,
            label: None,
        });
        self
    }

    pub fn cmp_eq(&mut self, left: Register, right: Register) -> Register {
        let dst = self.program.alloc_register();
        self.program.add_instruction(LirInstruction {
            op: LirOp::CmpEq(left, right),
            dst: Some(dst),
            src1: Some(left),
            src2: Some(right),
            immediate: None,
            label: None,
        });
        dst
    }

    pub fn cmp_ne(&mut self, left: Register, right: Register) -> Register {
        let dst = self.program.alloc_register();
        self.program.add_instruction(LirInstruction {
            op: LirOp::CmpNe(left, right),
            dst: Some(dst),
            src1: Some(left),
            src2: Some(right),
            immediate: None,
            label: None,
        });
        dst
    }

    pub fn cmp_lt(&mut self, left: Register, right: Register) -> Register {
        let dst = self.program.alloc_register();
        self.program.add_instruction(LirInstruction {
            op: LirOp::CmpLt(left, right),
            dst: Some(dst),
            src1: Some(left),
            src2: Some(right),
            immediate: None,
            label: None,
        });
        dst
    }

    pub fn cmp_le(&mut self, left: Register, right: Register) -> Register {
        let dst = self.program.alloc_register();
        self.program.add_instruction(LirInstruction {
            op: LirOp::CmpLe(left, right),
            dst: Some(dst),
            src1: Some(left),
            src2: Some(right),
            immediate: None,
            label: None,
        });
        dst
    }

    pub fn cmp_gt(&mut self, left: Register, right: Register) -> Register {
        let dst = self.program.alloc_register();
        self.program.add_instruction(LirInstruction {
            op: LirOp::CmpGt(left, right),
            dst: Some(dst),
            src1: Some(left),
            src2: Some(right),
            immediate: None,
            label: None,
        });
        dst
    }

    pub fn cmp_ge(&mut self, left: Register, right: Register) -> Register {
        let dst = self.program.alloc_register();
        self.program.add_instruction(LirInstruction {
            op: LirOp::CmpGe(left, right),
            dst: Some(dst),
            src1: Some(left),
            src2: Some(right),
            immediate: None,
            label: None,
        });
        dst
    }

    // Arithmetic Operations
    pub fn add(&mut self, left: Register, right: Register) -> Register {
        let dst = self.program.alloc_register();
        self.program.add_instruction(LirInstruction {
            op: LirOp::Add(left, right),
            dst: Some(dst),
            src1: Some(left),
            src2: Some(right),
            immediate: None,
            label: None,
        });
        dst
    }

    pub fn sub(&mut self, left: Register, right: Register) -> Register {
        let dst = self.program.alloc_register();
        self.program.add_instruction(LirInstruction {
            op: LirOp::Sub(left, right),
            dst: Some(dst),
            src1: Some(left),
            src2: Some(right),
            immediate: None,
            label: None,
        });
        dst
    }

    pub fn mul(&mut self, left: Register, right: Register) -> Register {
        let dst = self.program.alloc_register();
        self.program.add_instruction(LirInstruction {
            op: LirOp::Mul(left, right),
            dst: Some(dst),
            src1: Some(left),
            src2: Some(right),
            immediate: None,
            label: None,
        });
        dst
    }

    pub fn div(&mut self, left: Register, right: Register) -> Register {
        let dst = self.program.alloc_register();
        self.program.add_instruction(LirInstruction {
            op: LirOp::Div(left, right),
            dst: Some(dst),
            src1: Some(left),
            src2: Some(right),
            immediate: None,
            label: None,
        });
        dst
    }

    pub fn mod_op(&mut self, left: Register, right: Register) -> Register {
        let dst = self.program.alloc_register();
        self.program.add_instruction(LirInstruction {
            op: LirOp::Mod(left, right),
            dst: Some(dst),
            src1: Some(left),
            src2: Some(right),
            immediate: None,
            label: None,
        });
        dst
    }

    pub fn neg(&mut self, value: Register) -> Register {
        let dst = self.program.alloc_register();
        self.program.add_instruction(LirInstruction {
            op: LirOp::Neg(value),
            dst: Some(dst),
            src1: Some(value),
            src2: None,
            immediate: None,
            label: None,
        });
        dst
    }

    // Logical Operations
    pub fn and(&mut self, left: Register, right: Register) -> Register {
        let dst = self.program.alloc_register();
        self.program.add_instruction(LirInstruction {
            op: LirOp::And(left, right),
            dst: Some(dst),
            src1: Some(left),
            src2: Some(right),
            immediate: None,
            label: None,
        });
        dst
    }

    pub fn or(&mut self, left: Register, right: Register) -> Register {
        let dst = self.program.alloc_register();
        self.program.add_instruction(LirInstruction {
            op: LirOp::Or(left, right),
            dst: Some(dst),
            src1: Some(left),
            src2: Some(right),
            immediate: None,
            label: None,
        });
        dst
    }

    pub fn not(&mut self, value: Register) -> Register {
        let dst = self.program.alloc_register();
        self.program.add_instruction(LirInstruction {
            op: LirOp::Not(value),
            dst: Some(dst),
            src1: Some(value),
            src2: None,
            immediate: None,
            label: None,
        });
        dst
    }

    // Graph Operations
    pub fn create_node(&mut self, name: &str) -> Register {
        let dst = self.program.alloc_register();
        self.program.add_instruction(LirInstruction {
            op: LirOp::CreateNode(name.to_string()),
            dst: Some(dst),
            src1: None,
            src2: None,
            immediate: None,
            label: None,
        });
        dst
    }

    pub fn connect(&mut self, left: Register, right: Register) -> &mut Self {
        self.program.add_instruction(LirInstruction {
            op: LirOp::Connect(left, right),
            dst: None,
            src1: Some(left),
            src2: Some(right),
            immediate: None,
            label: None,
        });
        self
    }

    pub fn merge(&mut self, left: Register, right: Register) -> &mut Self {
        self.program.add_instruction(LirInstruction {
            op: LirOp::Merge(left, right),
            dst: None,
            src1: Some(left),
            src2: Some(right),
            immediate: None,
            label: None,
        });
        self
    }

    pub fn delete_node(&mut self, node: Register) -> &mut Self {
        self.program.add_instruction(LirInstruction {
            op: LirOp::DeleteNode(node),
            dst: None,
            src1: Some(node),
            src2: None,
            immediate: None,
            label: None,
        });
        self
    }

    // Context & State Operations
    pub fn push_ctx(&mut self) -> &mut Self {
        self.program.add_instruction(LirInstruction {
            op: LirOp::PushCtx,
            dst: None,
            src1: None,
            src2: None,
            immediate: None,
            label: None,
        });
        self
    }

    pub fn pop_ctx(&mut self) -> &mut Self {
        self.program.add_instruction(LirInstruction {
            op: LirOp::PopCtx,
            dst: None,
            src1: None,
            src2: None,
            immediate: None,
            label: None,
        });
        self
    }

    pub fn set_symbol(&mut self, symbol: &str, value: Register) -> &mut Self {
        self.program.add_instruction(LirInstruction {
            op: LirOp::SetSymbol(symbol.to_string(), value),
            dst: None,
            src1: Some(value),
            src2: None,
            immediate: None,
            label: None,
        });
        self
    }

    pub fn get_symbol(&mut self, symbol: &str) -> Register {
        let dst = self.program.alloc_register();
        self.program.add_instruction(LirInstruction {
            op: LirOp::GetSymbol(symbol.to_string()),
            dst: Some(dst),
            src1: None,
            src2: None,
            immediate: None,
            label: None,
        });
        dst
    }

    pub fn copy_ctx(&mut self) -> &mut Self {
        self.program.add_instruction(LirInstruction {
            op: LirOp::CopyCtx,
            dst: None,
            src1: None,
            src2: None,
            immediate: None,
            label: None,
        });
        self
    }

    // External Interface Operations
    pub fn call(&mut self, func_name: &str, args: Vec<Register>) -> Register {
        let dst = self.program.alloc_register();
        self.program.add_instruction(LirInstruction {
            op: LirOp::Call(func_name.to_string(), args),
            dst: Some(dst),
            src1: None,
            src2: None,
            immediate: None,
            label: None,
        });
        dst
    }

    pub fn read_io(&mut self, io_name: &str) -> Register {
        let dst = self.program.alloc_register();
        self.program.add_instruction(LirInstruction {
            op: LirOp::ReadIo(io_name.to_string()),
            dst: Some(dst),
            src1: None,
            src2: None,
            immediate: None,
            label: None,
        });
        dst
    }

    pub fn write_io(&mut self, io_name: &str, value: Register) -> &mut Self {
        self.program.add_instruction(LirInstruction {
            op: LirOp::WriteIo(io_name.to_string(), value),
            dst: None,
            src1: Some(value),
            src2: None,
            immediate: None,
            label: None,
        });
        self
    }

    // Special Operations
    pub fn phi(&mut self, dst: Register, values: Vec<Register>) -> &mut Self {
        self.program.add_instruction(LirInstruction {
            op: LirOp::Phi(dst, values),
            dst: Some(dst),
            src1: values.first().copied(),
            src2: values.get(1).copied(),
            immediate: None,
            label: None,
        });
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lir_builder_basic() {
        let mut builder = LirBuilder::new();
        
        // Build a simple program: load 5, load 3, add them
        let reg1 = builder.load_num(5);
        let reg2 = builder.load_num(3);
        let result = builder.add(reg1, reg2);
        
        let program = builder.build();
        
        assert_eq!(program.instructions.len(), 3);
        assert_eq!(result.id(), 2); // Third register allocated
    }

    #[test]
    fn test_lir_builder_control_flow() {
        let mut builder = LirBuilder::new();
        
        // Build a simple conditional: if x > 5 then print("large") else print("small")
        let x = builder.load_num(10);
        let five = builder.load_num(5);
        let cond = builder.cmp_gt(x, five);
        
        let then_label = 1;
        let end_label = 2;
        
        builder.jmp_if_not(cond, then_label);
        builder.label(then_label);
        builder.load_sym("print");
        builder.call("print", vec![builder.load_sym("large")]);
        builder.jmp(end_label);
        builder.label(end_label);
        
        let program = builder.build();
        
        assert!(program.instructions.len() >= 6); // Should have several instructions
    }
}