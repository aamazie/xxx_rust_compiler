use cranelift::prelude::*;
use cranelift_module::{Module, Linkage};
use cranelift_object::{ObjectBackend, ObjectBuilder};
use target_lexicon::Triple;
use crate::ir::{Instruction, IRProgram};

pub fn compile_ir_to_object(ir: IRProgram) -> anyhow::Result<Vec<u8>> {
    let triple = Triple::host();
    let builder = ObjectBuilder::new(
        cranelift::codegen::isa::lookup(triple.clone())?.finish(settings::builder()),
        "jit",
        cranelift_module::default_libcall_names(),
    )?;
    let mut module = cranelift_module::Module::new(builder);

    let mut ctx = module.make_context();
    let mut builder_ctx = FunctionBuilderContext::new();
    ctx.func.signature.returns.push(AbiParam::new(types::I32));

    let mut func_builder = FunctionBuilder::new(&mut ctx.func, &mut builder_ctx);
    let entry_block = func_builder.create_block();
    func_builder.append_block_params_for_function_params(entry_block);
    func_builder.switch_to_block(entry_block);
    func_builder.seal_block(entry_block);

    for instr in ir.instructions {
        match instr {
            Instruction::Return(val) => {
                let val = func_builder.ins().iconst(types::I32, val as i64);
                func_builder.ins().return_(&[val]);
            }
        }
    }

    func_builder.finalize();
    let id = module.declare_function("main", Linkage::Export, &ctx.func.signature)?;
    module.define_function(id, &mut ctx)?;
    module.clear_context(&mut ctx);
    module.finalize_definitions();

    let product = module.finish();
    Ok(product.emit()?)
}
