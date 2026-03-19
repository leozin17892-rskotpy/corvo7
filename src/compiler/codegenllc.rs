use crate::compiler::ir::*;
use crate::compiler::parser::Type;
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
        c.push_str("void print_i128(__int128 n) {
    if(n==0){printf(\"0\"); return;}
    if(n<0){printf(\"-\"); n=-n;}
    char str[40];
    int i=0;
    while(n>0){str[i++]=(n%10)+'0'; n/=10;}
    while(i--) putchar(str[i]);
    putchar('\\n');
}\n\n");

        // Mapa de temporários
    for fun in &ir.functions {

    // 🔹 abrir função
   	 if fun.name == "main" {
       	 writeln!(c, "int main() {{").unwrap();
   	 } else {
      	  writeln!(c, "void {}() {{", fun.name).unwrap();
   	 }
	
    	let mut declared_temps = std::collections::HashSet::new();
    	let mut declared_vars = std::collections::HashSet::new();

    // 🔹 1ª passada — coletar
    	for block in &fun.blocks {
      	  for instr in &block.instructions {
         	   match instr {
              	  Instr::ConstInt { dst, .. }
              	  | Instr::ConstBool { dst, .. }
              	  | Instr::Add { dst, .. }
             	   | Instr::Greater { dst, .. }
             	   | Instr::GreaterEq { dst, .. }
              	  | Instr::Less { dst, .. }
               	 | Instr::LessEq { dst, .. }
               	 | Instr::Load { dst, .. } => {
                    declared_temps.insert((dst.0, dst.clone().1));
                }

                Instr::Store { var, src } => {
                    declared_vars.insert((var.0, src.clone().1));
                }

                _ => {}
            }
        }
    }

    // 🔹 declarar no topo
    for v in &declared_vars {
        let fnt = match v.1{
            Type::Int => "int64_t",
            Type::Bool => "bool",
            Type::UInt => "uint64_t",
            Type::I8 => "int8_t",
            Type::I16 => "int16_t",
            Type::I32 => "int32_t",
            Type::I64 => "int64_t",
            Type::I128 => "int128_t",
            Type::U8 => "uint8_t",
            Type::U16 => "uint16_t",
            Type::U32 => "uint32_t",
            Type::U64 => "uint64_t",
            Type::U128 => "unsigned int128_t",
            Type::Vec{ .. } => "brh",
            Type::Void => "void",
            Type::Str => "char*",
            Type::Unknown => "n",
            _ => unreachable!(),
        };
        writeln!(c, "    {} v{};", fnt, v.0).unwrap();
    }

    for t in &declared_temps {
        let fnt = match t.1{
            Type::Int => "int64_t",
            Type::Bool => "bool",
            Type::UInt => "uint64_t",
            Type::I8 => "int8_t",
            Type::I16 => "int16_t",
            Type::I32 => "int32_t",
            Type::I64 => "int64_t",
            Type::I128 => "__int128",
            Type::U8 => "uint8_t",
            Type::U16 => "uint16_t",
            Type::U32 => "uint32_t",
            Type::U64 => "uint64_t",
            Type::U128 => "unsigned __int128",
            Type::Vec{ .. } => "brh",
            Type::Void => "void",
            Type::Str => "char*",
            Type::Unknown => "n",
            _ => unreachable!()
        };
        writeln!(c, "    {} t{};", fnt, t.0).unwrap();
    }

    writeln!(c).unwrap();

    // 🔹 2ª passada — emitir corpo
    for block in &fun.blocks {
        writeln!(c, "{}:", block.label.0).unwrap();

        for instr in &block.instructions {
            match instr {
                Instr::ConstInt { dst, value } => {
                    writeln!(c, "    t{} = {};", dst.0, value).unwrap();
                }

                Instr::ConstBool { dst, value } => {
                    writeln!(c, "    t{} = {};", dst.0, value).unwrap();
                }

                Instr::Store { var, src } => {
                    writeln!(c, "    v{} = t{};", var.0, src.0).unwrap();
                }

                Instr::Add { dst, lhs, rhs } => {
                    writeln!(c, "    t{} = t{} + t{};", dst.0, lhs.0, rhs.0).unwrap();
                }

                Instr::Greater { dst, lhs, rhs } => {
                    writeln!(c, "    t{} = t{} > t{};", dst.0, lhs.0, rhs.0).unwrap();
                }

                Instr::GreaterEq { dst, lhs, rhs } => {
                    writeln!(c, "    t{} = t{} >= t{};", dst.0, lhs.0, rhs.0).unwrap();
                }

                Instr::Less { dst, lhs, rhs } => {
                    writeln!(c, "    t{} = t{} < t{};", dst.0, lhs.0, rhs.0).unwrap();
                }

                Instr::LessEq { dst, lhs, rhs } => {
                    writeln!(c, "    t{} = t{} <= t{};", dst.0, lhs.0, rhs.0).unwrap();
                }

                Instr::Load { dst, id } => {
                    writeln!(c, "    t{} = v{};", dst.0, id.0).unwrap();
                }
                Instr::Print { temp } => {
                    let fmt = match &temp.1{
                        Type::Int => "%d\\n",
                        Type::Str => "%s\\n",
                        Type::Bool => "%bool",
                        _ => "%ld\\n"
                    };
                    match fmt{
                        "%d\\n" | "%s\\n" => writeln!(c, "	printf(\"{}\", t{});", fmt, temp.0).unwrap(),
                        "%bool" => writeln!(c, "	printf(\"%s\\n\", t{} ? \"true\" : \"false\");", temp.0).unwrap(),
                        "%ld\\n" => writeln!(c, "	printf(\"{}\", t{});", fmt, temp.0).unwrap(),
                        _ => unreachable!()
                    }
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

                _ => {}
            }
        }
    }

    writeln!(c, "}}\n").unwrap();
	}
    c
	}
}