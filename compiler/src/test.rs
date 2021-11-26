#[cfg(test)]
mod test {

    use crate::handler::select_compiler;
    use crate::data::input_data::compiler_type::CompilerType;
    //use crate::compilers::cpp_compiler::CppCompiler;
    

    fn convert_type_to_string<T>(_: &T) -> String {
        //println!("{}", std::any::type_name::<T>())
        let type_name = format!("{}", std::any::type_name::<T>());
        type_name
    }

    /* TODO fix type checking
    ---- test::test::select_cpp_compiler stdout ----
    thread 'test::test::select_cpp_compiler' panicked at 'assertion failed: `(left == right)`
    left: `"alloc::boxed::Box<dyn compiler::compilers::compiler::Compiler>"`,
    right: `"compilers::cpp_compiler::CppCompiler"`', src/test.rs:21:9
    note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
    */

    #[test]
    fn select_cpp_compiler() {
        
        let compiler = select_compiler(&CompilerType::Cpp); 
        
        assert_eq!(convert_type_to_string(&compiler), "compilers::cpp_compiler::CppCompiler");
    }

    
} 