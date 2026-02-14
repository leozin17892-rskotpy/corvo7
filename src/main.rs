mod compiler;

use compiler::lexer::lexer;
use compiler::parser::Parser;
use compiler::semantic::SemanticAnalyzer;
use compiler::codegen::Codegen;
use std::env;
use std::fs;
use std::process::Command;

fn main(){
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        eprintln!("Usage: {} <file.c7>", args[0]);
        std::process::exit(1);
    }
    
    let filename = &args[1];
    
    // Lexer (passa o filepath)
    let tokens = lexer(filename);
    
    // Parser
    let mut parser = Parser::new(tokens);
    let parse_result = parser.parse();
    let stmts = parse_result.unwrap_or_exit();
    
    // Semantic Analysis
    let mut analyzer = SemanticAnalyzer::new();
    let analysis = analyzer.analyze(&stmts);
    analysis.unwrap_or_exit();
    
    println!("✅ Semantic analysis");
    println!("Generating C code ya dumbass");
    let codegen = Codegen;
    let c_code = codegen.generate(&stmts);
    let output_filename = filename.replace(".c7", ".c");
    fs::write(&output_filename, &c_code)
    .expect("error writing on file.");
    let exe_name = filename.replace(".c7", "");
    
    let status = Command::new("clang")
        .arg(&output_filename)
        .arg("-o")
        .arg(&exe_name)
        .status();
    
    match status {
        Ok(exit_status) if exit_status.success() => {
            println!("🎉 Compilation successful ya genius!");
            println!("🚀 Running: ./{}", exe_name);
            println!("─────────────────────────────────");
            
            // ✅ CORREÇÃO AQUI
            let run_status = Command::new(format!("./{}", exe_name))
                .output();
            
            match run_status {
                Ok(exit) if exit.status.success() => {
                    print!("{}", String::from_utf8_lossy(& exit.stdout));
                    println!("✅ Ya did it! Program exited successfully.");
                    println!("─────────────────────────────────");
                }
                Ok(exit) => {
                    eprintln!("stderr: {}", String::from_utf8_lossy(&exit.stderr));
       			 println!("⚠️  Program failed with code: {:?}", exit.status.code());
                    println!("─────────────────────────────────");
           		 std::process::exit(1);
                }
                Err(e) => {
                    eprintln!("💔 Your code is so trash that the runner didn't want to run it! Error: {}", e);
                }
            }
        }
        Ok(out) => {
            eprintln!("❌ Clang said your code is trash. Fix it.");
        }
        Err(_) => {
            eprintln!("⚠️  Clang not found. Compile manually: clang {} -o {}", output_filename, exe_name);
        }
    }
}