use lang_c::driver::{Config, parse};
use lang_c::ast::{TranslationUnit, ExternalDeclaration, FunctionDefinition, Statement, Expression};
use crate::ir::{Instruction, IRProgram};
use anyhow::{Result, bail};

pub fn parse_c_to_ir(code: &str) -> Result<IRProgram> {
    let config = Config::default();
    let ast = parse(&config, code)
        .map_err(|e| anyhow::anyhow!("Parse error: {:?}", e))?;

    let mut instructions = Vec::new();

    for ext_decl in ast.0 {
        if let ExternalDeclaration::FunctionDefinition(func_def) = ext_decl {
            if let Some(body_instrs) = parse_function_body(&func_def) {
                instructions.extend(body_instrs);
            }
        }
    }

    Ok(IRProgram { instructions })
}

fn parse_function_body(func_def: &FunctionDefinition) -> Option<Vec<Instruction>> {
    let mut instructions = Vec::new();

    for stmt in &func_def.statement.0 {
        if let Statement::Return(Some(expr)) = stmt {
            if let Expression::Constant(constant) = expr {
                if let lang_c::ast::Constant::Int(val) = constant {
                    if let Ok(value) = val.to_string().parse::<i32>() {
                        instructions.push(Instruction::Return(value));
                    }
                }
            }
        }
    }

    if instructions.is_empty() {
        None
    } else {
        Some(instructions)
    }
}
