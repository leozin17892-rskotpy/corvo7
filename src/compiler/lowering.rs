use crate::compiler::parser::*;
use crate::compiler::ir::{BasicBlock, FunctionIR, IRProgram, Instr, Label, Temp, VarId};
use crate::compiler::parser::Type;
use crate::compiler::parser::Expr;
use std::collections::HashMap;


pub struct Lowering {
    temp_counter: usize,
    label_counter: usize,
    symbols: HashMap<VarId, Type>,
    var_table: HashMap<String, VarId>,
    next_var_id: usize,
}

impl Lowering {
    pub fn new() -> Self {
        Self {
            temp_counter: 0,
            label_counter: 0,
            symbols: HashMap::new(),
            var_table: HashMap::new(),
            next_var_id: 0
        }
    }
    fn block_terminated(block: &BasicBlock) -> bool {
   	 matches!(
       	 block.instructions.last(),
       	 Some(Instr::Return(_)) | Some(Instr::Jump(_)) | Some(Instr::JumpIfFalse { .. })
  	  )
	}

    fn new_temp(&mut self, ty: Type) -> Temp {
        let t = Temp::new(self.temp_counter, ty);
        self.temp_counter += 1;
        t
    }
    fn new_block(&mut self, blocks: &mut Vec<BasicBlock>) -> usize {
 	   let label = self.new_label();
  	  let idx = blocks.len();
        
	    blocks.push(BasicBlock {
      	  label,
       	 instructions: Vec::new(),
    	});

    	idx
	}
    fn new_label(&mut self) -> Label {
        let label = Label(self.label_counter);
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
    pub fn lower_expr(&mut self, expr: &Expr, cur_block: usize, blocks: &mut Vec<BasicBlock>) -> Temp {
        match expr {
            Expr::IntLiteral(v) => {
                let dst = self.new_temp(Type::Int);
                blocks[cur_block].instructions.push(Instr::ConstInt { dst: dst.clone(), value: *v as i128 });
                dst
            }
            Expr::BoolLiteral(v) => {
                let dst = self.new_temp(Type::Bool);
                blocks[cur_block].instructions.push(Instr::ConstBool { dst: dst.clone(), value: *v });
                dst
            }
            Expr::Ident(name) => {
                let var_id = self.get_var_id(name);
                let ty = self.symbols.get(&var_id)
                .expect("Variable with no Type")
                .clone();
                let dst = self.new_temp(ty);
                blocks[cur_block].instructions.push(Instr::Load { dst: dst.clone(), id: var_id });
                dst
            }
            Expr::BinaryOp { left, op, right } => {
                let l = self.lower_expr(left, cur_block, blocks);
                let r = self.lower_expr(right, cur_block, blocks);
                
                assert!(l.ty == r.ty, "Operands type are invalid on BinaryOperation");
                let dst_ty = match op {
 				   BinOp::Add | BinOp::Sub | BinOp::Mul | BinOp::Div => l.ty.clone(),
  				  BinOp::Greater | BinOp::Less | BinOp::GreaterEqual | BinOp::LessEqual => Type::Bool,
   				 _ => todo!(),
				};
                let dst = self.new_temp(dst_ty);
                match (op, l.ty()){
                    (BinOp::Add, ty) if ty.is_int()=> blocks[cur_block].instructions.push(Instr::Add { dst: dst.clone(), lhs: l, rhs: r }),
                    (BinOp::Sub, ty) if ty.is_int() => blocks[cur_block].instructions.push(Instr::Sub { dst: dst.clone(), lhs: l, rhs: r }),
                    (BinOp::Mul, ty) if ty.is_int() => blocks[cur_block].instructions.push(Instr::Mul { dst: dst.clone(), lhs: l, rhs: r }),
                    (BinOp::Div, ty) if ty.is_int() => blocks[cur_block].instructions.push(Instr::Div { dst: dst.clone(), lhs: l, rhs: r }),
                    
                    (BinOp::Greater, ty) if ty.is_int() => blocks[cur_block].instructions.push(Instr::Greater { dst: dst.clone(), lhs: l, rhs: r }),
                    (BinOp::GreaterEqual, ty) if ty.is_int() => blocks[cur_block].instructions.push(Instr::GreaterEq { dst: dst.clone(), lhs: l, rhs: r }),
                    (BinOp::Less, ty) if ty.is_int() => blocks[cur_block].instructions.push(Instr::Less { dst: dst.clone(), lhs: l, rhs: r }),
                    (BinOp::LessEqual, ty) if ty.is_int() => blocks[cur_block].instructions.push(Instr::LessEq { dst: dst.clone(), lhs: l, rhs: r }),
                    _ => todo!("Operador binário não implementado"),
                }
                dst
            }
            _ => panic!("Expr não implementada ainda: {:?}", expr),
        }
    }

    /// Lowering de statements
    pub fn lower_stmt(
        &mut self,
        stmt: &Stmt,
        cur_block: usize,
        blocks: &mut Vec<BasicBlock>,
    ) -> usize{
        match stmt {
            Stmt::VarDecl(var) => {
                let val = self.lower_expr(&var.value, cur_block, blocks);
                let var_id = self.get_var_id(&var.name);
                self.symbols.insert(var_id, val.ty.clone());
                blocks[cur_block].instructions.push(Instr::Store { var: var_id, src: val });
                return cur_block;
            }
            Stmt::Assignment(assign) => {
                let val = self.lower_expr(&assign.value, cur_block, blocks);
                let var_id = self.get_var_id(&assign.target);
                blocks[cur_block].instructions.push(Instr::Store { var: var_id, src: val });
                return cur_block;
            }
            Stmt::Print(exprs) => {
                for expr in exprs {
                    let temp = self.lower_expr(expr, cur_block, blocks); // ou self.get_type(expr) se quiser
                    blocks[cur_block].instructions.push(Instr::Print { temp });
                }
                return cur_block;
            }
            Stmt::IfStatement(if_stmt) => {
                if Self::block_terminated(&blocks[cur_block]) {
			    	return cur_block;
				}
  			  let cond = self.lower_expr(&if_stmt.condition, cur_block, blocks);

 			   let then_block = self.new_block(blocks);
 			   let else_block = self.new_block(blocks);
 			   let cont_block = self.new_block(blocks);
                
                let else_label = blocks[else_block].label.clone();
				let then_label = blocks[then_block].label.clone();

				blocks[cur_block].instructions.push(
  				  Instr::JumpIfFalse { cond, label: else_label }
				);
				blocks[cur_block].instructions.push(
   				 Instr::Jump(then_label)
				);
  			  // THEN
  			  let mut then_cur = then_block;
  			  for stmt in &if_stmt.then_branch {
      			  then_cur = self.lower_stmt(stmt, then_cur, blocks);
  			  }
                let cont_label = blocks[cont_block].label.clone();
                if !Self::block_terminated(&blocks[then_cur]) {
				    blocks[then_cur].instructions.push(
    			    Instr::Jump(cont_label)
				    );
				}
    // ELSE
  			  let mut else_cur = else_block;
  			  if let Some(else_branch) = &if_stmt.else_branch {
    			    for stmt in else_branch {
          			  else_cur = self.lower_stmt(stmt, else_cur, blocks);
     			   }
 			   }
				if !Self::block_terminated(&blocks[else_cur]) {
                    let cont_label = blocks[cont_block].label.clone();
 				   blocks[else_cur].instructions.push(
      			 	 Instr::Jump(cont_label)
   				 );
				}

  			  cont_block
			}
            Stmt::Return(expr) => {
                if Self::block_terminated(&blocks[cur_block]){
                	return cur_block;
                }
                let val = self.lower_expr(expr, cur_block, blocks);
                blocks[cur_block].instructions.push(Instr::Return(Some(val)));
                cur_block
            }
            _ => todo!("Stmt não implementado ainda"),
        }
    }

    /// Lowering de programa
    pub fn lower_program(&mut self, stmts: &[Stmt], name: &str) -> IRProgram {
        let mut blocks = Vec::new();
        let mut cur = self.new_block(&mut blocks);

		for stmt in stmts {
 		   cur = self.lower_stmt(stmt, cur, &mut blocks);
		}

        let func = FunctionIR { name: name.to_string(), blocks };
        IRProgram { functions: vec![func] }
    }
}