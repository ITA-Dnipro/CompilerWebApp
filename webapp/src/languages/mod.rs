// Information about available languages
pub mod static_info;
// Struct for flags validation based on a given language
pub mod flags_validator;
// Information about a language
pub mod lang_info;
// Trait for flags parsing
mod flags_parser;
// Enumeration of compiler flags types and necessary implementations
mod compiler_flag;

// Tests module
#[cfg(test)]
mod flags_validation_tests;
