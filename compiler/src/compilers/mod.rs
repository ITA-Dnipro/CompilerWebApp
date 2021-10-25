pub mod cpp_compiler;
pub mod rust_compiler;

trait Compiler {
    fn Compile(&self, input_data: InputData) -> OutputData;
}