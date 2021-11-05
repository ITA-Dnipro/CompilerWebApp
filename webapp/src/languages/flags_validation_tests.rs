use super::static_info::cpp;
use super::flags_validator::FlagsValidator;
use super::compiler_flag::CompilerFlag as cf;

#[test]
fn valid_cpp_flags() {
    let cpp_info = cpp::construct();
    assert!(FlagsValidator::validate(
        "-std=default", 
        &cpp_info))
}

#[test]
fn prohibited_cpp_flags() {
    let cpp_info = cpp::construct();
    assert!(!FlagsValidator::validate(
        "-o somebadfolder/hehe.exe -key=value -key2=morevalue=case", 
        &cpp_info))
}

#[test]
fn undefined_equal() {
    assert!(cf::Undefined == cf::UndefinedFlag("val".to_owned()));
}
