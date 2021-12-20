use serde::Serialize;
use std::collections::HashMap;


#[derive(FromForm, Serialize, Debug)]
pub struct Context
{
    pub settings: Vec<HashMap<String, String>> //Vec<ConfigHashMap<'t>>
}

impl Context
{
    
}