pub mod cpp;

use std::collections::HashSet;
use super::compiler_flag::CompilerFlag;

pub trait FlagsParser
{
    // Parse a string into a set of flags
    fn parse(&self, flags_as_text: &str) -> HashSet<CompilerFlag>;
}
