// Information about available languages
pub mod static_info;
// Struct for flags validation based on a given language
pub mod flags_validator;
// Trait for flags parsing
mod flags_parser;
// Enumeration of compiler flags types and necessary implementations
mod compiler_flag;
// Information about a language
mod lang_info;

// Tests module
#[cfg(test)]
mod flags_validation_tests;
