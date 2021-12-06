#[cfg(test)]
mod test {

    use crate::handler::select_compiler;
    use crate::data::input_data::compiler_type::CompilerType;
    use crate::options::extract_key_and_value;


    fn convert_type_to_string<T>(_: &T) -> String {
        let type_name = format!("{}", std::any::type_name::<T>());
        type_name
    }

    #[test]
    fn select_cpp_compiler() {
        let compiler = select_compiler(&CompilerType::Cpp); 
        
        assert_eq!(convert_type_to_string(&compiler), "compilers::cpp_compiler::CppCompiler");
    }

    

    #[test]
    fn extract_key_and_value__OK_single_key () {
        let mut key: String;
        let mut value: String;
        
        let option = String::from("-v"); 
        
        let extraction_result = extract_key_and_value(&option);

        match extraction_result {
            Ok(key_value) => {
                key = key_value.0;
                value = key_value.1;                
            }

            Err(_e) => {
                
            }
        }

        assert_eq!(key, "-v");
        assert_eq!(value, "");
    }

} 
