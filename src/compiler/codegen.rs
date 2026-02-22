use crate::compiler::parser::*;
use std::collections::HashMap;
use std::cell::RefCell;


pub struct Codegen {
    pub symbols: RefCell<HashMap<String, Type>>,
}

impl Codegen {
    pub fn new() -> Self{
        Self { symbols: RefCell::new(HashMap::new()) }
    }
    pub fn generate(&self, stmts: &[Stmt]) -> String {
        // No seu Codegen::generate, adicione isso ao Header:
		let mut output = String::from("#include <stdint.h>
#include <stdio.h>
#include <inttypes.h>
#include <stdbool.h>


void print_i128(__int128 n) {
    if (n == 0) { printf(\"0\"); return; }
    if (n < 0) { printf(\"-\"); n = -n; }
    char str[40]; 
    int i = 0;
    while (n > 0) {
       str[i++] = (n % 10) + '0';
       n /= 10;
    }
    while (i--) putchar(str[i]);
}\n\n
");
        for stmt in stmts {
            output.push_str(&self.gen_stmt(stmt));
            output.push('\n');
        }
        output
    }
    fn indent_with(&self, spaces: usize, code: &str) -> String {
    	let prefix = " ".repeat(spaces);
   	 code.lines()
    	    .map(|l| if l.trim().is_empty() { l.to_string() } else { format!("{}{}", prefix, l) })
    	    .collect::<Vec<_>>()
    	    .join("\n")
	}

    fn is_comparison_op(&self, op: &BinOp) -> bool {
   	 matches!(op, 
    	    BinOp::DoubleEqual |
      	  BinOp::NotEqual |      // se tiver
      	  BinOp::Greater |
      	  BinOp::GreaterEqual |
     	   BinOp::Less |
        	BinOp::LessEqual |
      	  BinOp::IndentityOp
   	 )
	}


    fn gen_format_cast(&self, expr: &Expr) -> (&'static str, Option<String>, bool){
        match expr{
            Expr::IntLiteral(v) => {
                if *v >= i8::MIN as i64 && *v <= i8::MAX as i64{
                    ("%d", None, false)
                }else if *v >= i16::MIN as i64 && *v <= i16::MAX as i64{
                    ("%\" PRId16 \"", None, false)
                }else if *v >= i32::MIN as i64 && *v <= i32::MAX as i64{
                    ("%\" PRId32 \"", None, false)
                }else {
                    ("%\" PRId64 \"", None, false)
                }
            }
            Expr::Ident(name) => {
            	let symbols = self.symbols.borrow();
           	 if let Some(ty) = symbols.get(name) {
             	   match ty {
                	    Type::I8  => ("%d", None, false),
                  	  Type::I16 => ("%\" PRId16 \"", None, false),
                   	 Type::I32 => ("%\" PRId32 \"", None, false),
                  	  Type::I64 => ("%\" PRId64 \"", None, false),
                        Type::I128 => ("special_i128", None, false),
                        Type::U8 =>  ("%u", None, false),
                        Type::U16 => ("%\" PRIu16 \"", None, false),
                   	 Type::U32 => ("%\" PRIu32 \"", None, false),
                        Type::U64 => ("%\" PRIu64 \"", None, false),
                        Type::U128 => ("special_u128", None, false),
                        Type::Str => ("\"%s\"", None, false),
                  	  Type::Bool => ("%s", None, true), // Para o ternário true/false
                  	  _ => ("%d", None, false),
               	 }
            	} else {
               	 ("%d", None, false) // Fallback
            	}
        	}
            Expr::UIntLiteral(v) => {
                if *v >= u8::MIN as u64 && *v <= u8::MAX as u64{
                    ("%u", None, false)
                }else if *v >= u16::MIN as u64 && *v <= u16::MAX as u64{
                    ("%\" PRIu16 \"", None, false)
                }else if *v >= u32::MIN as u64 && *v <= u32::MAX as u64{
                    ("%\" PRIu32 \"", None, false)
                }else {
                    ("%\" PRIu64\"", None, false)
                }
            }
            Expr::I128Literal(v) => ("special_i128", None, false),
            Expr::U128Literal(v) => ("special_u128", None, false),
            Expr::BoolLiteral(b) => {
                ("%s", None, true)
            }
            Expr::BinaryOp { op, .. } => {
    		if self.is_comparison_op(op) {
    	 	   ("%s", None, true)
   		 } else {
       		 ("%\" PRId64 \"", Some("int64_t".to_string()), false)
   		 }
		}
       	 Expr::StringLiteral(_) => ("%s", None, false),
     	   _ => ("%d", None, false)
        }
    }
    fn gen_stmt(&self, stmt: &Stmt) -> String {
        match stmt {
            Stmt::Print(exprs) => {
   			 let mut code = String::new();
    			for expr in exprs {
        			let val = self.gen_expr(expr);
       			 let (fmt, _cast, is_ternary) = self.gen_format_cast(expr);

       		 if fmt == "special_i128" {
          		  code.push_str(&format!("print_i128({}); ", val));
       	 } else if is_ternary {
        	    code.push_str(&format!("printf(\"%s \", ({}) ? \"true\" : \"false\"); ", val));
       	 } else {
         	   code.push_str(&format!("printf(\"{} \", {}); ", fmt, val));
       	 }
  	  }
  		  code.push_str("printf(\"\\n\");"); // Quebra de linha no final
  		  code
		}

            Stmt::CompoundAssign { target, op, value } => {
                let mut code = String::new();
            	let op_str = match op {
              	  BinOp::CompoundAdd => "+=",
            	    BinOp::CompoundSub => "-=",
             	   BinOp::CompoundMul => "*=",
           	     BinOp::CompoundDiv => "/=",
              	  _ => panic!("Operador composto inválido no codegen"),
           	 };
         	   let val_str = self.gen_expr(value);
                code.push_str(&format!("{} {} {};", target, op_str, val_str));
                self.indent_with(4, &code)
        }
            Stmt::Assignment(a) => {
                self.indent_with(4, &format!("{} = {};", &a.target, self.gen_expr(&a.value)))
            }
            Stmt::VarDecl(v) => {
                let prefix = match v.mutability{
                    Mutability::Const => "const ",
      			  Mutability::Mutable => "",
                };
                let code = match &v.ty{
                    Type::Vec { inner, size } => {
                        let inner_ty = Self::gen_type(inner);
                        let val = self.gen_expr(&v.value);
                        format!("{} {} {}[{}] = {};", prefix, inner_ty, v.name, size, val)
                    }
                    _ => {
                        let ty = Self::gen_type(&v.ty);
                		let val = self.gen_expr(&v.value);
                        self.symbols.borrow_mut().insert(v.name.clone(), v.ty.clone());
                        format!("{}{} {} = {};", prefix, ty, v.name, val)
                    }
                };
                code
            }
            Stmt::Break => "break;".to_string(),
            Stmt::Continue => "continue;".to_string(),
            Stmt::FuncDecl(f) => {
                let ret_ty = Self::gen_type(&f.return_type.as_ref().unwrap_or(&Type::Void));
                
                // Correção do Erro E0401: Usamos Codegen:: em vez de Self dentro do map
                let params: Vec<String> = f.params.iter()
                    .map(|(ty, name)| format!("{} {}", Self::gen_type(ty), name))
                    .collect();
                
                let mut body = String::new();
                for s in &f.body {
                    body.push_str("    ");
                    body.push_str(&self.gen_stmt(s));
                    body.push('\n');
                }
                let has_return = f.body.iter().any(|s| matches!(s, Stmt::Return(_)));
                if f.name == "main" && matches!(f.return_type, Some(Type::Int)) && !has_return{
                    body.push_str("    return 0;\n");
                }
                format!("{} {}({}) {{\n{}}}", ret_ty, f.name, params.join(", "), body)
            }
            // Correção do Erro de Sintaxe: Removido o 'return' e o ';' extra
            Stmt::Return(e) => format!("return {};", self.gen_expr(e)),
            Stmt::Loop(l) => {
                let mut body_code = String::new();
                for s in &l.body{
                    body_code.push_str(&self.gen_stmt(s));
                    body_code.push('\n');
                }
                format!(
                    "while(1)\n    {{\n{}\n    }}",
                    self.indent_with(8, &body_code)
                )
            }
            Stmt::ForLoop(u) => {
                let start_code = self.gen_expr(&u.start);
                let step_code = if let Some(step) = &u.step{
                   	 self.gen_expr(step)
   				 } else {
    			 	   "1".to_string() // step implícito
 		 		  };
                let cmp = if step_code.starts_with("-") { ">=" } else { "<=" };
                let end_code = self.gen_expr(&u.end);
                let mut body_code = String::new();
                for s in &u.body{
                    body_code.push_str(&self.gen_stmt(s));
                    body_code.push('\n');
                }
                format!(
                    "for (int {} = {}; {} {} {}; {} += {})\n    {{\n{}\n    }}",
                    u.var,
                    start_code,
                    u.var,
                    cmp,
                    end_code,
                    u.var,
                    step_code,
                    self.indent_with(8, &body_code),
                )
            }
            Stmt::WhileLoop(w) => {
                let cond = self.gen_expr(&w.cond);
                let mut body_code = String::new();
                for s in &w.body{
                    body_code.push_str(&self.gen_stmt(s));
                    body_code.push('\n');
                }
                format!(
                    "while({}) {{\n{}\n    }}",
                    cond,
                    self.indent_with(8, &body_code)
                )
            }
            Stmt::WhenStatement(w) => {
                let cond = self.gen_expr(&w.condition);
                let mut out = format!("switch({}){{\n", cond);
                for (value_expr, block) in &w.arms {
       			 let value = self.gen_expr(value_expr); 
       				 out.push_str(&format!("    case {}:\n", value));
        

      			  for stmt in block {
           			 let line = self.gen_stmt(stmt); 
           			 out.push_str(&format!("        {};\n", line));
       			 }

      			  out.push_str("        break;\n");
  		    }

    
    		if let Some(block) = &w.else_arm {
       		 out.push_str("    default:\n");
       		 for stmt in block {
        	   	 let line = self.gen_stmt(stmt);
         	 	  out.push_str(&format!("        {};\n", line));
      		  }
       		 out.push_str("        break;\n");
   		 }
    			out.push_str("}\n");

   			 out
        }        
            Stmt::IfStatement(i) => {
                let cond = self.gen_expr(&i.condition);
                let mut out = format!("if({}){{\n", cond);
                for s in &i.then_branch {
                    out.push_str(&format!("       {}\n", self.gen_stmt(s)));
                }
                out.push_str("    }");
    
                if let Some(else_b) = &i.else_branch {
                    if else_b.len() == 1 && matches!(else_b[0], Stmt::IfStatement(_)) {
       				 out.push_str("else "); // Não abre chaves!
      				  out.push_str(&self.gen_stmt(&else_b[0])); // Gera o próximo if colado
   				 } else {
        // Else normal com chaves
        				out.push_str("else {\n");
      				  for s in else_b {
         				   out.push_str(&format!("       {}\n", self.gen_stmt(s)));
      				  }
                        out.push_str("    }");
                  }
             }
             out
            }		  // Retorno do braço do match
          }
        } // Fim do match (Sem ponto e vírgula aqui para ele ser o retorno da função)

    fn gen_expr(&self, expr: &Expr) -> String {
        match expr {
            Expr::UIntLiteral(v) => v.to_string(),
            Expr::IntLiteral(v) => v.to_string(),
            Expr::I128Literal(v) => {
                let high = v >> 64;
   			 let low = v & ((1 << 64) - 1);
  			  format!("((__int128){} << 64 | (__int128){}ULL)", high, low)
            },
            Expr::U128Literal(v) => {
                let high = v >> 64;
   			 let low = v & ((1 << 64) - 1);
  			  format!("((unsigned __int128){} << 64 | (unsigned __int128){})", high, low)
            },
            Expr::BoolLiteral(v) => v.to_string(),
            Expr::StringLiteral(v) => format!("\"{}\"", v),
            Expr::Ident(s) => s.clone(),
            Expr::Vec { values, size: _ } => {
                if values.is_empty(){
                    "{0}".to_string()
                }else{
                    let elems: Vec<String> = values.iter()
                    .map(|e| self.gen_expr(e))
                    .collect();
                    format!("{{{}}}", elems.join(", "))
                }
            }
            Expr::BinaryOp { left, op, right } => {
                let op_str = match op {
                    BinOp::Add => "+".to_string(),
                    BinOp::Sub => "-".to_string(),
                    BinOp::Mul => "*".to_string(),
                    BinOp::Div => "/".to_string(),
                    BinOp::DoubleEqual => "==".to_string(),
                    BinOp::Greater => ">".to_string(),
                    BinOp::GreaterEqual => ">=".to_string(),
                    BinOp::Less => "<".to_string(),
                    BinOp::LessEqual => "<=".to_string(),
                    BinOp::Percent => "%".to_string(),
                    BinOp::IndentityOp => "==".to_string(),
                    _ => "unknown".to_string()
                };
                // Correção E0401: Chamada direta
                format!("{} {} {}", self.gen_expr(left), op_str, &self.gen_expr(right))
            }
            Expr::Identity { expr, negated } => {
 			   let inner = self.gen_expr(expr);

   			 if *negated {
      			  format!("(!{})", inner)
  			  } else {
      			  format!("({})", inner)
    			}
              }
			
            Expr::Call { name, args } => {
                let args_str: Vec<String> = args.iter().map(|e| self.gen_expr(e)).collect();
                format!("{}({})", name, args_str.join(", "))
            }
            _ => "0".to_string(), 
        }
   }

    fn gen_type(ty: &Type) -> String {
        match ty {
            Type::U8 => "uint8_t".to_string(),
            Type::U16 => "uint16_t".to_string(),
            Type::U32 => "uint32_t".to_string(),
            Type::U64 => "uint64_t".to_string(),
            Type::UInt => "uint64_t".to_string(),
            Type::U128 => "unsigned __int128".to_string(),
            Type::I8 => "int8_t".to_string(),
            Type::I16 => "int16_t".to_string(),
            Type::I32 => "int32_t".to_string(),
            Type::I64 => "int64_t".to_string(),
            Type::I128 => "__int128".to_string(),
            Type::Bool => "bool".to_string(),
            Type::Str => "char*".to_string(),
            Type::Void => "void".to_string(),
            Type::Int => "int".to_string(),
            _ => "int".to_string(), 
        }
    }
}
