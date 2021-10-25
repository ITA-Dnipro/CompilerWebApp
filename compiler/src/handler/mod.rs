use ::std::boxed::Box;

use super::data::input_data::InputData;
use super::data::input_data::compiler_type::CompilerType;
use super::data::output_data::OutputData;
use super::compilers::cpp_compiler::CppCompiler;
use super::compilers::rust_compiler::RustCompiler;
use super::compilers::compiler::Compiler;


// main process

pub fn RunCompilation(input_data: InputData) -> OutputData {
    

    let compiler = SelectCompiler(input_data.header.compiler_type);
    let output_data: OutputData = compiler.Compile(input_data);
    output_data
}

fn SelectCompiler(compiler_type: CompilerType) -> Box<dyn Compiler> {
    
    
    match compiler_type {
        CompilerType::Cpp => {
            Box::new(CppCompiler {})
        }

        CompilerType::Rust => {
            Box::new(RustCompiler {})
        }

        _ => {
            Box::new(CppCompiler {})
        }
    } 
}
