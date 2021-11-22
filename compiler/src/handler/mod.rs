use std::collections::HashMap;

use ::std::boxed::Box;

use super::data::input_data::InputData;
use super::data::input_data::compiler_type::CompilerType;
use super::data::output_data::OutputData;
use super::compilers::cpp_compiler::CppCompiler;
use super::compilers::rust_compiler::RustCompiler;
use super::compilers::compiler::Compiler;
use super::options::{parse_compiler_options, filter_compiler_options};


const OPTIONS_SEPARATOR: &str = r" ";

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
/// # Example
///
/// ```ignore
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
pub fn run_compilation(input_data: &InputData) -> Result<OutputData, &'static str> {
    
    let mut updated_input_data = InputData::new(
        input_data.compiler_type.to_owned(), 
        input_data.source_code_file_path.to_owned(), 
        input_data.compiled_directory_path.to_owned(), 
        input_data.compiler_options.to_owned());

  
    
    // Get options as string
    let raw_options = input_data.compiler_options.clone();
    // TODO add to logger
    //println!("Example string: {}", BOTH_OPTIONS_EXAMPLE);
    
    // Get whitelist of options
    // TODO use YAML config - figment crate
    let mut options_whitelist: Vec<String> = Vec::new();
    options_whitelist.push("-v".to_string());
    options_whitelist.push("-Wall".to_string());

    // Split options by "space"
    let options: Vec<String> = raw_options.split(OPTIONS_SEPARATOR).map(|s| s.to_string()).collect();

    // Parse each option ad extract key-s from "key and value" pairs or only "key"
    let parsing_result = parse_compiler_options(&options);

    
    match parsing_result {
        Ok(mut parsed_options) => {
            let options_keys: Vec<String> = parsed_options.keys().map(|s| s.to_string()).collect();
            
            let filtering_result =  filter_compiler_options(&options_keys, &options_whitelist);
    
            match filtering_result {
                Ok(declined_keys) => {
                    let filtered_options: HashMap<String, String> = parsed_options.drain_filter(|k, _v| declined_keys.contains(k)).collect();           

                    // TODO add to logger
                    /*
                    println!("Accepted options list:");
            
                    for option in filtered_options {
                        println!("Option key: {}, option value: {}", option.0, option.1);
                    }
                    */

                    let filtered_options_vector: Vec<String> = filtered_options.into_iter().map(|(key, value)| format!("{}={}", key, value)).collect();
                    let filtered_options_string: String = filtered_options_vector.join(" ");

                    updated_input_data.compiler_options = filtered_options_string;
                }
        
                Err(_declined_keys) => {
                    // TODO add to logger
                    /*
                    println!("Declined options list:");
            
                    for option in filtered_options {
                        println!("Option: {}", option);
                    }
                    */
                    return Err("Filtering options failed.")
                }
            }
        }

        Err(e) => {
            // TODO add to logger
            //println!("Parsing error: {}", e);
            return Err(e);
        }
    }



    
    let compiler = select_compiler(&updated_input_data.compiler_type);

    let output_data = compiler.compile(&updated_input_data)?;
    Ok(output_data)
}


/// Does selection of compiler depending on compiler type
///
/// # Arguments
///
/// * `compiler_type` - A enum valut that contains one of the possible type of compiler
///
/// # Result
///
/// * A box that holds selected compiler
/// 
/// 
/// # Example
///
/// ```ignore
/// use super::data::input_data::compiler_type::CompilerType;
/// 
/// pub fn run_compilation(input_data: &InputData) -> OutputData {
///     let compiler = select_compiler(&input_data.compiler_type);
///     let output_data: OutputData = compiler.compile(input_data);
///     output_data
/// }
/// 
pub(crate) fn select_compiler(compiler_type: &CompilerType) -> Box<dyn Compiler> {
    match compiler_type {
        CompilerType::Cpp => {
            Box::new(CppCompiler {})
        }

        CompilerType::Rust => {
            Box::new(RustCompiler {})
        }
    } 
}
