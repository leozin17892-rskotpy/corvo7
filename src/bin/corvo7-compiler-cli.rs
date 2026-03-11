use corvo7::compiler::lexer::lexer;
use corvo7::compiler::parser::Parser as CParser;
use corvo7::compiler::semantic::SemanticAnalyzer;
use corvo7::compiler::codegen::Codegen;
use corvo7::compiler::codegenllc::CodegenLLC;
use corvo7::compiler::lowering::Lowering;

use std::path::Path;
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
    
    #[arg(short, long, default_value_t = false)]
    ll: bool,
    
    #[arg(short, long, default_value_t = 2)]
    O: u8,

    /// Apenas compilar, não rodar
    #[arg(short, long, default_value_t = false)]
    compile_only: bool,
}
fn run() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    let filename = Path::new(&cli.filename);

    // Lexer
    let tokens = match lexer(&cli.filename) {
    Ok(tokens) => tokens,
    Err(errs) => {
        return Err(format!(
            "aborting due to {} previous errors",
            errs.0.len()
        ).into());
  	  }
	};

    // Parser
    let mut parser = CParser::new(tokens);
    let stmts = parser.parse().unwrap_or_exit();

    let mut analyzer = SemanticAnalyzer::new();
    analyzer.analyze(&stmts).unwrap_or_exit();

	let c_code = if !cli.ll {
    	let mut codegen = Codegen::new();
   	 codegen.generate(&stmts)
	} else {
  	  let codegen = CodegenLLC::new();
 	   let mut lower = Lowering::new();
   	 let lowered = lower.lower_program(&stmts, "main");
  	  codegen.generate(&lowered)
	};

    let output_c = filename.with_extension("c");
    fs::write(&output_c, &c_code)?;
    
    let compiler = if cli.gcc { "gcc" } else { "clang" };
    
    #[cfg(target_os = "windows")]
    let output_exe = filename.with_extension("exe");
    
    #[cfg(not(target_os = "windows"))]
    let output_exe = filename.with_extension("");
    
    let status = Command::new(compiler)
    .arg(&output_c)
    .arg("-o")
    .arg(&output_exe)
    .status()?;
    let otimization_level = if cli.O <= 5 { cli.O } else { 2 };
    let mut extra_flag = "";
    if otimization_level >= 4{
        extra_flag = "-march=native";
    }
    let output = Command::new(compiler)
    .arg(&output_c)
    .arg("-o")
    .arg(&output_exe)
    .arg(&format!("-O{}", otimization_level))
    .arg(extra_flag)
    .output()?;
    if !status.success(){
        eprintln!("{}", String::from_utf8_lossy(&output.stderr));
        return Err(format!("{} failed to compile generated C code", compiler).into());
    }
    if !cli.compile_only{
        let start = Instant::now();
        let run_status = Command::new(&output_exe)
        .status()?;

   	 if !run_status.success() {
       	 return Err("program exited with error".into());
 	   }
        let end = start.elapsed();
        println!("program exited with {:.2?}", end);
    }
    Ok(())
}
fn main() {
    if let Err(err) = run(){
        eprintln!("{err}");
        std::process::exit(1);
    }
}