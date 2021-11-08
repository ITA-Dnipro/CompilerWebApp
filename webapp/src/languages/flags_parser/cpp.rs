use super::FlagsParser;
use std::collections::HashSet;
use super::super::compiler_flag::CompilerFlag;

pub struct CppParser
{}

impl FlagsParser for CppParser
{
    fn parse(&self, flags_as_text: &str) -> HashSet<CompilerFlag>
    {
        let words = flags_as_text.split_whitespace();
        let mut flags = HashSet::<CompilerFlag>::new();
        let mut chars;
        let mut cur_char;

        for word in words
        {
            chars = word.chars();
            cur_char = chars.next().unwrap();
            
            flags.insert(match cur_char
            {
                '-' => 
                {
                    let parts: Vec<&str> = word.split('=').collect();

                    match parts.len()
                    {
                        1 => { CompilerFlag::new_single_word(parts[0]) },
                        2 => { CompilerFlag::new_key_value(parts[0], parts[1]) },
                        _ => { CompilerFlag::UndefinedFlag(word.to_owned()) }
                    }
                },
                _ => { CompilerFlag::UndefinedFlag(word.to_owned()) }
            });
        }

        flags
    }
}
