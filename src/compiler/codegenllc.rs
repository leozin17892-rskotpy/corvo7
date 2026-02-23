use crate::compiler::ir::*;
use std::fmt::Write;

pub struct CodegenLLC;

impl CodegenLLC {
    pub fn new() -> Self {
        Self
    }

    pub fn generate(&self, ir: &IRProgram) -> String {
        let mut c = String::new();

        // Cabeçalho C
        c.push_str("#include <stdio.h>\n#include <stdbool.h>\n#include <stdint.h>\n\n");
        c.push_str("int main() {\n");

        // Mapa de temporários
        let mut declared_temps = std::collections::HashSet::new();
        let mut declared_vars = std::collections::HashSet::new();
        for instr in &ir.instructions {
            match instr {
       	 Instr::ConstInt { dst, .. } 
        	| Instr::ConstBool { dst, .. }
       	 | Instr::Add { dst, .. }
        	| Instr::Sub { dst, .. }
       	 | Instr::Mul { dst, .. }
      	  | Instr::Div { dst, .. }
       	 | Instr::Greater { dst, .. }
            | Instr::Less { dst, ..}
            | Instr::LessEq { dst, ..}
            | Instr::GreaterEq { dst, ..}
      	  | Instr::Load { dst, .. } => {
          	  declared_temps.insert(dst.0);
       	 }
       	 Instr::Store { name, .. } => {
          	  declared_vars.insert(name.clone());
      	  }
       	 _ => {}
   		 }
		}

// Declarar todas as variáveis do usuário no topo
		for var in &declared_vars {
  		  writeln!(c, "    int64_t {} = 0;", var).unwrap();
		}
        for t in &declared_temps {
  	 	 writeln!(c, "    int64_t t{} = 0;", t).unwrap();
		}

        for instr in &ir.instructions {
            match instr {
                Instr::ConstInt { dst, value } => {
  				  if declared_temps.insert(dst.0) {
       				 writeln!(c, "    int64_t t{} = {};", dst.0, value).unwrap();
  				  } else {
    				    writeln!(c, "    t{} = {};", dst.0, value).unwrap();
    				}
				}

				Instr::ConstBool { dst, value } => {
   				 if declared_temps.insert(dst.0) {
       				 writeln!(c, "    bool t{} = {};", dst.0, value).unwrap();
  				  } else {
        				writeln!(c, "    t{} = {};", dst.0, value).unwrap();
  			 	 }
				}

				Instr::Add { dst, lhs, rhs } => {
  			  	if declared_temps.insert(dst.0) {
        				writeln!(c, "    int64_t t{} = t{} + t{};", dst.0, lhs.0, rhs.0).unwrap();
			    	} else {
    			   	 writeln!(c, "    t{} = t{} + t{};", dst.0, lhs.0, rhs.0).unwrap();
  			  	}
				}
                Instr::Sub { dst, lhs, rhs} => {
                    if declared_temps.insert(dst.0){
                        writeln!(c, "    int64_t t{} = t{} - t{};", dst.0, lhs.0, rhs.0).unwrap();
                    }else{
                        writeln!(c, "    t{} = t{} - t{};", dst.0, lhs.0, rhs.0).unwrap();
                    }
                }

				Instr::Greater { dst, lhs, rhs } => {
   				 if declared_temps.insert(dst.0) {
      			 	 writeln!(c, "    int64_t t{} = t{} > t{};", dst.0, lhs.0, rhs.0).unwrap();
   				 } else {
       				 writeln!(c, "    t{} = t{} > t{};", dst.0, lhs.0, rhs.0).unwrap();
  				  }
				}
                Instr::GreaterEq { dst, lhs, rhs } => {
                    if declared_temps.insert(dst.0) {
                        writeln!(c, "    int64_t t{} = t{} >= t{};", dst.0, lhs.0, rhs.0).unwrap();
                    }else{
                        writeln!(c, "    t{} = t{} >= t{};", dst.0, lhs.0, rhs.0).unwrap();
                    }
                }
                Instr::Less { dst, lhs, rhs } => {
                    if declared_temps.insert(dst.0){
                        writeln!(c, "    int64_t t{} = t{} < t{};", dst.0, lhs.0, rhs.0).unwrap();
                    }else{
                        writeln!(c, "    t{} = t{} < t{};", dst.0, lhs.0, rhs.0).unwrap();
                    }
                }
                Instr::LessEq { dst, lhs, rhs } => {
                    if declared_temps.insert(dst.0){
                        writeln!(c, "    int64_t t{} = t{} <= t{};", dst.0, lhs.0, rhs.0).unwrap();
                    }else{
                        writeln!(c, "    t{} = t{} <= t{};", dst.0, lhs.0, rhs.0).unwrap();
                    }
                }

				Instr::Load { dst, name } => {
   				 if declared_temps.insert(dst.0) {
     			 	  writeln!(c, "    int64_t t{} = {};", dst.0, name).unwrap();
  			 	 } else {
      			 	 writeln!(c, "    t{} = {};", dst.0, name).unwrap();
  			 	 }
				}
                Instr::Store { name, src } => {
 				   writeln!(c, "    {} = t{};", name, src.0).unwrap();
				}
                Instr::Label(label) => {
                    writeln!(c, "{}:", label.0).unwrap();
                }
                Instr::Jump(label) => {
                    writeln!(c, "    goto {};", label.0).unwrap();
                }
                Instr::JumpIfFalse { cond, label } => {
                    writeln!(c, "    if (!t{}) goto {};", cond.0, label.0).unwrap();
                }
                Instr::Return(Some(temp)) => {
                    writeln!(c, "    return t{};", temp.0).unwrap();
                }
                Instr::Return(None) => {
                    writeln!(c, "    return 0;").unwrap();
                }
                _ => {
                    // Call, Eq e outras ops podem ser adicionadas depois
                    writeln!(c, "    // {:?} não implementado", instr).unwrap();
                }
            }
        }
        c.push_str("    return 0;\n");
        c.push_str("}\n");
        c
    }
}