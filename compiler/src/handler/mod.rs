use ::std::boxed::Box;

use super::data::input_data::InputData;
use super::data::input_data::compiler_type::CompilerType;
use super::data::output_data::OutputData;
use super::compilers::cpp_compiler::CppCompiler;
use super::compilers::rust_compiler::RustCompiler;
use super::compilers::compiler::Compiler;


// main process
pub fn run_compilation(input_data: &InputData) -> OutputData {
    

    let compiler = select_compiler(&input_data.compiler_type);
    let output_data: OutputData = compiler.compile(input_data);
    output_data
}

fn select_compiler(compiler_type: &CompilerType) -> Box<dyn Compiler> {
    match compiler_type {
        CompilerType::Cpp => {
            Box::new(CppCompiler {})
        }

        CompilerType::Rust => {
            Box::new(RustCompiler {})
        }
    } 
}
