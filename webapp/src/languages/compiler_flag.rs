#![allow(unused)]
use std::collections::HashSet;
use std::hash::{Hash, Hasher};

#[derive(Eq, Clone)]
pub enum CompilerFlag   // Types of flags a compiler may take
{
    // An undefined flag
    Undefined,
    // An undefined flag that stores it's text
    UndefinedFlag(String),
    // A flag that consists of a single word
    SingleWordFlag(String),
    // A flag that consists of a key-value pair
    KeyValueFlag
    {
        key: String,
        value: String
    },
    // A flag that has many values for a single key
    KeyValuesFlag
    {
        key: String,
        values: HashSet<String>
    },
}

impl CompilerFlag
{
    pub fn new_single_word(flag: &str) -> CompilerFlag
    {
        CompilerFlag::SingleWordFlag(flag.to_owned())
    }

    pub fn new_key_value(key: &str, value: &str) -> CompilerFlag
    {
        CompilerFlag::KeyValueFlag 
        {
            key: key.to_owned(),
            value: value.to_owned()
        }
    }

    pub fn new_key_values(key: &str, values: &[&str]) -> CompilerFlag
    {
        let mut new_values = HashSet::<String>::new();
        for value in values
        {
            new_values.insert(value.to_string());
        }

        CompilerFlag::KeyValuesFlag
        {
            key: key.to_owned(),
            values: new_values
        }
    }
}

impl PartialEq for CompilerFlag
{
    fn eq(&self, other: &CompilerFlag) -> bool
    {
        use CompilerFlag::*;
        // Compare by keys and values
        match (self, other)
        {
            (Undefined, Undefined) => true,
            (Undefined, UndefinedFlag(_)) => true,
            (UndefinedFlag(_), Undefined) => true,

            (UndefinedFlag(self_value), UndefinedFlag(other_value)) => 
            { 
                self_value == other_value
            }
            (SingleWordFlag(self_flag), SingleWordFlag(other_flag)) =>
            {
                self_flag == other_flag
            },
            (KeyValueFlag { key: self_key, value: self_value },  
                KeyValueFlag { key: other_key, value: other_value }) =>
            {
                self_key == other_key && self_value == other_value
            },

            // One to many comparison here is true if a range contains the element it's compared to
            (KeyValueFlag { key: self_key, value: self_value },  
                KeyValuesFlag { key: other_key, values: other_values }) =>
            {
                self_key == other_key && other_values.contains(self_value)
            },
            (KeyValuesFlag { key: self_key, values: self_values },  
                KeyValueFlag { key: other_key, value: other_value }) =>
            {
                self_key == other_key && self_values.contains(other_value)
            },
            (KeyValuesFlag { key: self_key, values: self_values },  
                KeyValuesFlag { key: other_key, values: other_values }) =>
            {
                self_key == other_key && self_values == other_values
            },
            _ => false
        }
    }
}

impl Hash for CompilerFlag
{
    fn hash<H: Hasher>(&self, hasher: &mut H)
    {
        // Don't hash by a values of keys
        match self
        {   
            CompilerFlag::Undefined => 
            {
                0.hash(hasher);
            }
            CompilerFlag::UndefinedFlag(value) =>
            {
                value.hash(hasher);
            }
            CompilerFlag::SingleWordFlag(flag) =>
            {
                flag.hash(hasher);
            },
            CompilerFlag::KeyValueFlag { key, value: _ } =>
            {
                key.hash(hasher);
            },
            CompilerFlag::KeyValuesFlag { key, values: _ } =>
            {
                key.hash(hasher);
            }
        }
    }
}

impl ToString for CompilerFlag
{
    fn to_string(&self) -> String
    {
        match self
        {
            CompilerFlag::Undefined => "Undefined".to_owned(),
            CompilerFlag::UndefinedFlag(text) => text.to_owned(),
            CompilerFlag::SingleWordFlag(flag) => flag.to_owned(),
            CompilerFlag::KeyValueFlag { key, value } => format!("{}={}", key, value),
            CompilerFlag::KeyValuesFlag { key, values} => format!("{}={}", key,
                values.iter().map(|val| val.to_owned() + "|").collect::<String>())
        }
    }
}
