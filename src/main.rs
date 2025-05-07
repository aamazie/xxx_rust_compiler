mod ir;
mod codegen;
mod parser;

use std::env;
use std::fs;
use codegen::compile_ir_to_object;
use parser::parse_c_to_ir;

fn main() -> anyhow::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: {} <input.c> <output.o>", args[0]);
        std::process::exit(1);
    }

    let source = fs::read_to_string(&args[1])?;
    let ir = parse_c_to_ir(&source)?;
    let obj = compile_ir_to_object(ir)?;
    fs::write(&args[2], obj)?;
    println!("Output written to {}", args[2]);
    Ok(())
}
