#![allow(unused)]
use compiler::data::input_data::compiler_type::CompilerType::Cpp;
use super::super::
{
    lang_info::LangInfo, compiler_flag::
    {
        CompilerFlag as Flag
    }
};
use std::collections::HashSet;
use super::super::flags_parser::cpp::CppParser;

// Retuns LangInfo with information about c++
pub fn construct() -> LangInfo
{
    // List of prohibited flags
    let flags = [
        Flag::Undefined,
        Flag::new_single_word("-o")
    ];
    let mut flags_set = HashSet::new();

    for flag in flags
    {
        flags_set.insert(flag);
    }

    LangInfo::new(Cpp, ".cpp", false, flags_set, Box::new(CppParser {}))
}
