use ::std::boxed::Box;

use super::data::input_data::InputData;
use super::data::input_data::compiler_type::CompilerType;
use super::data::output_data::OutputData;
use super::compilers::cpp_compiler::CppCompiler;
use super::compilers::rust_compiler::RustCompiler;
use super::compilers::compiler::Compiler;


/// Runs main compilation process
///
/// # Arguments
///
/// * `input_data` - A struct that holds input parameters for compiler (source code, compiler options, etc)
///
/// # Result
///
/// * A struct that holds compiled binary file and specific compiler output (stdout, stderr)
/// 
/// 
/// # Examples
///
/// ```
/// 
/// use std::path::PathBuf;
/// use compiler::handler::run_compilation;
/// 
/// let input_data = InputData {
///     compiler_type: CompilerType::Cpp,
///     source_code_file_path: PathBuf::from("./temp/src/test.cpp"),
///     compiled_directory_path: PathBuf::from("./temp/bin/"), 
///     compiler_options: String::from("-g"),
/// };
/// 
/// let output_data = run_compilation(&input_data);
/// 
/// println!(">> Compiling status: {}", output_data.status_code.unwrap());
/// println!(">> Compiled file path: {}", output_data.compiled_file_name.into_os_string().into_string().unwrap());
/// println!(">> Value of stdout:");
/// io::stdout().write_all(&output_data.stdout).unwrap();
/// println!(">> Value of stderr:");
/// io::stderr().write_all(&output_data.stderr).unwrap();
/// 
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
