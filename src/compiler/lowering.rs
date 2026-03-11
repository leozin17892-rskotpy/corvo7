use crate::compiler::parser::*;
use crate::compiler::ir::{BasicBlock, FunctionIR, IRProgram, Instr, Label, Temp, VarId};
use crate::compiler::parser::Type;
use crate::compiler::parser::Expr;
use std::collections::HashMap;


pub struct Lowering {
    temp_counter: usize,
    label_counter: usize,
    break_stack: Vec<Label>,
    continue_stack: Vec<Label>,
    var_table: HashMap<String, VarId>,
    next_var_id: usize,
}

impl Lowering {
    pub fn new() -> Self {
        Self {
            temp_counter: 0,
            label_counter: 0,
            break_stack: Vec::new(),
            continue_stack: Vec::new(),
            var_table: HashMap::new(),
            next_var_id: 0
        }
    }

    fn new_temp(&mut self, ty: Type) -> Temp {
        let t = Temp(self.temp_counter, ty);
        self.temp_counter += 1;
        t
    }
    fn new_label(&mut self, prefix: &str) -> Label {
        let label = Label(format!("{}_{}", prefix, self.label_counter));
        self.label_counter += 1;
        label
    }
    fn get_var_id(&mut self, name: &str) -> VarId {
        if let Some(id) = self.var_table.get(name) {
            id.clone()
        } else {
            let id = VarId(self.next_var_id);
            self.next_var_id += 1;
            self.var_table.insert(name.to_string(), id.clone());
            id
        }
    }

    /// Lowering de expressões
    pub fn lower_expr(&mut self, expr: &Expr, cur_block: &mut BasicBlock) -> Temp {
        match expr {
            Expr::IntLiteral(v) => {
                let dst = self.new_temp(Type::Int);
                cur_block.instructions.push(Instr::ConstInt { dst: dst.clone(), value: *v as i128 });
                dst
            }
            Expr::BoolLiteral(v) => {
                let dst = self.new_temp(Type::Bool);
                cur_block.instructions.push(Instr::ConstBool { dst: dst.clone(), value: *v });
                dst
            }
            Expr::Ident(var_id) => {
                let dst = self.new_temp(Type::Str);
                let var_id_usz = self.get_var_id(var_id);
                cur_block.instructions.push(Instr::Load { dst: dst.clone(), id: var_id_usz });
                dst
            }
            Expr::BinaryOp { left, op, right } => {
                let l = self.lower_expr(left, cur_block);
                let r = self.lower_expr(right, cur_block);
                let dst = self.new_temp(Type::Str);
                match op {
                    BinOp::Add => cur_block.instructions.push(Instr::Add { dst: dst.clone(), lhs: l, rhs: r }),
                    BinOp::Sub => cur_block.instructions.push(Instr::Sub { dst: dst.clone(), lhs: l, rhs: r }),
                    BinOp::Mul => cur_block.instructions.push(Instr::Mul { dst: dst.clone(), lhs: l, rhs: r }),
                    BinOp::Div => cur_block.instructions.push(Instr::Div { dst: dst.clone(), lhs: l, rhs: r }),
                    BinOp::Greater => cur_block.instructions.push(Instr::Greater { dst: dst.clone(), lhs: l, rhs: r }),
                    BinOp::GreaterEqual => cur_block.instructions.push(Instr::GreaterEq { dst: dst.clone(), lhs: l, rhs: r }),
                    BinOp::Less => cur_block.instructions.push(Instr::Less { dst: dst.clone(), lhs: l, rhs: r }),
                    BinOp::LessEqual => cur_block.instructions.push(Instr::LessEq { dst: dst.clone(), lhs: l, rhs: r }),
                    _ => todo!("Operador binário não implementado"),
                }
                dst
            }
            _ => todo!("Expr não implementada ainda"),
        }
    }

    /// Lowering de statements
    pub fn lower_stmt(
        &mut self,
        stmt: &Stmt,
        cur_block: &mut BasicBlock,
        blocks: &mut Vec<BasicBlock>,
    ) {
        match stmt {
            Stmt::VarDecl(var) => {
                let val = self.lower_expr(&var.value, cur_block);
                let var_id = self.get_var_id(&var.name);
                cur_block.instructions.push(Instr::Store { var: var_id, src: val });
            }
            Stmt::Assignment(assign) => {
                let val = self.lower_expr(&assign.value, cur_block);
                let var_id = self.get_var_id(&assign.target);
                cur_block.instructions.push(Instr::Store { var: var_id, src: val });
            }
            Stmt::Print(exprs) => {
                for expr in exprs {
                    let temp = self.lower_expr(expr, cur_block); // ou self.get_type(expr) se quiser
                    cur_block.instructions.push(Instr::Print { temp });
                }
            }
            Stmt::IfStatement(if_stmt) => {
                let cond_temp = self.lower_expr(&if_stmt.condition, cur_block);

                let then_label = self.new_label("then");
                let else_label = self.new_label("else");
                let endif_label = self.new_label("endif");

                // jump condicional
                cur_block.instructions.push(Instr::JumpIfFalse { cond: cond_temp, label: else_label.clone() });

                // then block
                let mut then_block = BasicBlock { label: then_label.clone(), instructions: Vec::new() };
                for stmt in &if_stmt.then_branch {
                    self.lower_stmt(stmt, &mut then_block, blocks);
                }
                then_block.instructions.push(Instr::Jump(endif_label.clone()));
                blocks.push(then_block);

                // else block
                let mut else_block = BasicBlock { label: else_label.clone(), instructions: Vec::new() };
                if let Some(else_branch) = &if_stmt.else_branch {
                    for stmt in else_branch {
                        self.lower_stmt(stmt, &mut else_block, blocks);
                    }
                }
                else_block.instructions.push(Instr::Jump(endif_label.clone()));
                blocks.push(else_block);

                // bloco de continuação
                let cont_block = BasicBlock { label: endif_label.clone(), instructions: Vec::new() };
                blocks.push(cont_block);
            }
            Stmt::Return(expr) => {
                let val = self.lower_expr(expr, cur_block);
                cur_block.instructions.push(Instr::Return(Some(val)));
            }
            _ => todo!("Stmt não implementado ainda"),
        }
    }

    /// Lowering de programa
    pub fn lower_program(&mut self, stmts: &[Stmt], name: &str) -> IRProgram {
        let mut blocks = Vec::new();
        let mut entry_block = BasicBlock { label: Label("entry".to_string()), instructions: Vec::new() };

        for stmt in stmts {
            self.lower_stmt(stmt, &mut entry_block, &mut blocks);
        }

        blocks.insert(0, entry_block);

        let func = FunctionIR { name: name.to_string(), blocks };
        IRProgram { functions: vec![func] }
    }
}