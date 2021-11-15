#![allow(unused)]
use compiler::data::input_data::compiler_type::CompilerType;
use std::collections::HashSet;
use super::compiler_flag::CompilerFlag;
use super::flags_parser::FlagsParser;

// Storage of language info
pub struct LangInfo
{
    pub lang_type: CompilerType,
    pub lang_extension: String,
    // True for whitelist, false for blacklist
    pub are_flags_whitelist: bool,
    pub compiler_flags: HashSet<CompilerFlag>,
    pub parser: Box<dyn FlagsParser + Send + Sync>
}

impl LangInfo
{
    pub fn new(lang_type: CompilerType, lang_extension: &str, 
        are_flags_whitelist: bool, compiler_flags: HashSet<CompilerFlag>,
        parser: Box<dyn FlagsParser + Send + Sync>)
        -> LangInfo
    {
        LangInfo
        {
            lang_type,
            lang_extension: lang_extension.to_owned(),
            are_flags_whitelist,
            compiler_flags,
            parser
        }
    }
}
