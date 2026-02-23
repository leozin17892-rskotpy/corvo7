use crate::compiler::parser::*;
use crate::compiler::ir::*;


pub struct Lowering {
    temp_counter: usize,
    label_counter: usize,
    break_stack: Vec<String>,
    continue_stack: Vec<String>,
}

impl Lowering {
    pub fn new() -> Self {
        Self {
            temp_counter: 0,
            label_counter: 0,
            break_stack: Vec::new(),
            continue_stack: Vec::new(),
        }
    }
    fn new_temp(&mut self) -> Temp {
        let t = Temp(self.temp_counter);
        self.temp_counter += 1;
        t
    }

    fn new_label(&mut self, prefix: &str) -> String {
        let name = format!("{}_{}", prefix, self.label_counter);
        self.label_counter += 1;
        name
    }
    pub fn get_type(&self, expr: &Expr) -> Type {
        match expr {
            Expr::IntLiteral(_) => Type::Int,
            Expr::UIntLiteral(_) => Type::UInt,
            Expr::I128Literal(_) => Type::I128,
            Expr::U128Literal(_) => Type::U128,
            Expr::BoolLiteral(_) => Type::Bool,
            Expr::StringLiteral(_) => Type::Str,
            Expr::BinaryOp { left, right, op: _ } => {
                let left_ty = self.get_type(left);
                let right_ty = self.get_type(right);
                // Supondo coercion para tipos maiores
                self.coerce_type(left_ty, right_ty)
            }
            _ => Type::Int, // fallback genérico
        }
    }

    /// Coerção simples de tipo
    fn coerce_type(&self, a: Type, b: Type) -> Type {
        use Type::*;
        match (a, b) {
            (I128, _) | (_, I128) => I128,
            (U128, _) | (_, U128) => U128,
            (I64, _) | (_, I64) => I64,
            (U64, _) | (_, U64) => U64,
            (Int, _) | (_, Int) => Int,
            (UInt, _) | (_, UInt) => UInt,
            (Bool, Bool) => Bool,
            (Str, Str) => Str,
            _ => Int,
        }
    }
    fn lower_if(&mut self, if_stmt: &IfStatement, out: &mut Vec<Instr>){
   	 let cond_temp = self.lower_expr(&if_stmt.condition, out);

  	  let then_label = self.new_label("then");
  	  let else_label = self.new_label("else");
   	 let endif_label = self.new_label("endif");

    // jump if false
    	out.push(Instr::JumpIfFalse { cond: cond_temp, label: Label(else_label.clone()) });
        
        for stmt in &if_stmt.then_branch{
            self.lower_stmt(stmt, out);
        }
        
        out.push(Instr::Jump(Label(endif_label.clone())));
        
        out.push(Instr::Label(Label(else_label)));
        if let Some(else_branch) = &if_stmt.else_branch{
            for stmt in else_branch {
          	  self.lower_stmt(stmt, out);
       	 }
        }
        out.push(Instr::Label(Label(endif_label.clone())))
	}
    pub fn lower_expr(&mut self, expr: &Expr, out: &mut Vec<Instr>) -> Temp{
        match expr{
            Expr::IntLiteral(v) => {
                let dst = self.new_temp();
                out.push(Instr::ConstInt { dst, value: *v as i128 });
                dst
            }
            Expr::BoolLiteral(v) => {
                let dst = self.new_temp();
                out.push(Instr::ConstBool { dst, value: *v });
                dst
            }
            Expr::Ident(name) => {
                let dst = self.new_temp();
                out.push(Instr::Load { dst, name: name.clone() });
                dst
            }
            Expr::BinaryOp{left, op, right} => {
                let l = self.lower_expr(left, out);
                let r = self.lower_expr(right, out);
                let dst = self.new_temp();
                match op{
                    BinOp::Add => out.push(Instr::Add { dst, lhs: l, rhs: r }),
               	 BinOp::Sub => out.push(Instr::Sub { dst, lhs: l, rhs: r }),
              	  BinOp::Mul => out.push(Instr::Mul { dst, lhs: l, rhs: r }),
               	 BinOp::Div => out.push(Instr::Div { dst, lhs: l, rhs: r }),
                    BinOp::Greater => out.push(Instr::Greater { dst, lhs: l, rhs: r}),
                    BinOp::GreaterEqual => out.push(Instr::GreaterEq { dst, lhs: l, rhs: r}),
                    BinOp::Less => out.push(Instr::Less { dst, lhs: l, rhs: r}),
                    BinOp::LessEqual => out.push(Instr::LessEq { dst, lhs: l, rhs: r}),
              	  _ => todo!("op não implementado ainda"),
            		}
                dst
                }
            _ => todo!("não implementei doido")    
            }
        }
    fn lower_stmt(&mut self, stmt: &Stmt, out: &mut Vec<Instr>){
 	   match stmt {
     	   Stmt::VarDecl(var) => {
         	   let value = self.lower_expr(&var.value, out);
          	  out.push(Instr::Store {
            	    name: var.name.clone(),
            	    src: value,
          	  });
       	 }

        	Stmt::Assignment(assign) => {
           	 let value = self.lower_expr(&assign.value, out);
           	 out.push(Instr::Store {
               	 name: assign.target.clone(),
             	   src: value,
           	 });
            }
            Stmt::Print(exprs) => {
                for expr in exprs{
                    let temp = self.lower_expr(expr, out);
                    let ty = self.get_type(expr);
                    
                    out.push(Instr::Print { temp, ty });
                }
            }
            Stmt::IfStatement(if_stmt) => self.lower_if(if_stmt, out),

      	  Stmt::Return(expr) => {
          	  let value = self.lower_expr(expr, out);
         	   out.push(Instr::Return(Some(value)));
     	   }

     	   _ => todo!("stmt não implementado ainda"),
  	  }
	}
    pub fn lower_program(&mut self, stmts: &[Stmt]) -> IRProgram {
   	 let mut program = IRProgram { instructions: Vec::new() };

    	for stmt in stmts {
      	  self.lower_stmt(stmt, &mut program.instructions);
  	  }

   	 program
	}
}