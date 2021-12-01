#[cfg(test)]
mod test {

    use crate::handler::select_compiler;
    use crate::data::input_data::compiler_type::CompilerType;
 
    fn convert_type_to_string<T>(_: &T) -> String {
        let type_name = format!("{}", std::any::type_name::<T>());
        type_name
    }

    #[test]
    fn select_cpp_compiler() {
        let compiler = select_compiler(&CompilerType::Cpp); 
        
        assert_eq!(convert_type_to_string(&compiler), "compilers::cpp_compiler::CppCompiler");
    }
} 