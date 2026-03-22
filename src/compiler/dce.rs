use std::collections::{HashMap, HashSet};
use crate::compiler::ir::*;


pub fn eliminate_dead_temps(ir: &mut IRProgram) {
    for func in &mut ir.functions {
        // coleta usos em TODOS os blocos da função
        let mut used: HashSet<usize> = HashSet::new();

        for block in &func.blocks {
            for instr in &block.instructions {
                collect_uses(instr, &mut used);
            }
        }

        // depois elimina em cada bloco
        for block in &mut func.blocks {
            block.instructions.retain(|instr| match instr {
                Instr::ConstInt { dst, .. }
                | Instr::ConstBool { dst, .. }
                | Instr::Load { dst, .. }
                | Instr::Add { dst, .. }
                | Instr::Greater { dst, .. }
                | Instr::GreaterEq { dst, .. }
                | Instr::Less { dst, .. }
                | Instr::LessEq { dst, .. } => used.contains(&dst.id),
                _ => true,
            });
        }
    }
}

fn collect_uses(instr: &Instr, used: &mut HashSet<usize>) {
    match instr {
        Instr::Store { src, .. } => { used.insert(src.id); }
        Instr::Add { lhs, rhs, .. }
        | Instr::Greater { lhs, rhs, .. }
        | Instr::GreaterEq { lhs, rhs, .. }
        | Instr::Less { lhs, rhs, .. }
        | Instr::LessEq { lhs, rhs, .. } => {
            used.insert(lhs.id);
            used.insert(rhs.id);
        }
        Instr::Print { temp } => { used.insert(temp.id); }
        Instr::JumpIfFalse { cond, .. } => { used.insert(cond.id); }
        Instr::Return(Some(temp)) => { used.insert(temp.id); }
        _ => {}
    }
}