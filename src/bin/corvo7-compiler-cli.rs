use corvo7::compiler::lexer::lexer;
use corvo7::compiler::parser::Parser as CParser;
use corvo7::compiler::semantic::SemanticAnalyzer;
use corvo7::compiler::codegen::Codegen;

use std::env;
use std::fs;
use std::process::Command;
use std::time::Instant;

use clap::Parser;


/// Corvo7 Compiler CLI
#[derive(Parser)]
#[command(author="Léo", version="0.1", about="Compile and run Corvo7 programs", long_about = None)]
struct Cli {
    /// Arquivo Corvo7
    filename: String,

    /// Use GCC instead of Clang
    #[arg(short, long, default_value_t = false)]
    gcc: bool,

    /// Apenas compilar, não rodar
    #[arg(short, long, default_value_t = false)]
    compile_only: bool,
}

fn main() {
    let cli = Cli::parse();

    let filename = &cli.filename;

    // Lexer
    let tokens = lexer(filename);

    // Parser
    let mut parser = CParser::new(tokens);
    let stmts = parser.parse().unwrap_or_exit();

    // Semantic Analysis
    let mut analyzer = SemanticAnalyzer::new();
    analyzer.analyze(&stmts).unwrap_or_exit();

    println!("✅ Semantic analysis complete");
    println!("Generating C code...");

    let codegen = Codegen;
    let c_code = codegen.generate(&stmts);
    let output_c = filename.replace(".c7", ".c");
    fs::write(&output_c, &c_code).expect("Error writing C file");
    let exe_name = filename.replace(".c7", "");

    // Escolha do compilador
    let compiler = if cli.gcc { "gcc" } else { "clang" };

    let status = Command::new(compiler)
        .arg(&output_c)
        .arg("-o")
        .arg(&exe_name)
        .status()
        .expect("Failed to start compiler");

    if !status.success() {
        eprintln!("❌ Compilation failed with {}", compiler);
        std::process::exit(1);
    }

    println!("🎉 Compilation successful: {}", exe_name);

    if cli.compile_only {
        return; // não roda se --compile-only
    }

    println!("🚀 Running: ./{}", exe_name);
    println!("─────────────────────────────────");

    let start = Instant::now();
    let run_status = Command::new(format!("./{}", exe_name))
        .output()
        .expect("Failed to run executable");
    let duration = start.elapsed();

    if run_status.status.success() {
        print!("{}", String::from_utf8_lossy(&run_status.stdout));
        println!("✅ Program exited successfully.");
        println!("⏱️ Execution time: {:.6} seconds", duration.as_secs_f64());
        println!("─────────────────────────────────");
    } else {
        eprintln!("stderr: {}", String::from_utf8_lossy(&run_status.stderr));
        println!("⚠️  Program failed with code {:?}", run_status.status.code());
        println!("⏱️ Execution time: {:.6} seconds", duration.as_secs_f64());
        std::process::exit(1);
    }
}