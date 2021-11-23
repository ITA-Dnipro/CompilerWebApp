use std::collections::HashMap;

// TODO use typedef for compiler options
use regex::Regex;
use std::collections::HashSet;
use std::iter::FromIterator;

/*
const BOTH_OPTIONS_EXAMPLE: &str = concat!(
    "-a -bb -ccc -ddd -  -- --- --v --verbose", 
    " -a=va -bb=vbb -ccc=vccc -ddd=vddd -e=  -eee= --e= ---- -=- ====");
*/

fn to_hashset(vector: &Vec<String>) -> HashSet<String> {
    HashSet::from_iter(vector.iter().cloned())
}


// if there are one or more option in "options" from "whitelist"
//     - return Ok(declined_options)
// else (zero "accepted option")
//     - return Err(declined_options)
pub(crate) fn filter_compiler_options(options: &Vec<String>, options_whitelist: &Vec<String>) -> Result<Vec<String>, &'static str> {
    let options_set: HashSet<String>  = to_hashset(options);
    let options_whitelist_set: HashSet<String> = to_hashset(options_whitelist);

    let accepted_options_set: HashSet<&String>  = options_set.intersection(&options_whitelist_set).collect();
    let declined_options_set: HashSet<&String> =  options_set.difference(&options_whitelist_set).collect();

    let accepted_options: Vec<String> = accepted_options_set.into_iter().map(|s| s.to_string()).collect();    
    let declined_options: Vec<String> = declined_options_set.into_iter().map(|s| s.to_string()).collect();

    Ok(declined_options)  
}


pub(crate) fn parse_compiler_options(options: &Vec<String>) -> Result<HashMap<String, String>, &'static str> {
    if options.len() > 0 {
        let mut key_value_options: HashMap<String, String> = HashMap::new();

        let mut key: String;
        let mut value: String;

        for option in options {
            let extraction_result = extract_key_and_value(option);

            match extraction_result {
                Ok(key_value) => {
                    key = key_value.0;
                    value = key_value.1;

                    key_value_options.insert(key, value);
                }

                Err(_e) => {
                    // TODO process incorrect options
                    //return Err(e)
                }
            }
        }

        Ok(key_value_options)
    }    
    else {
        Err("Options vector is empty")
    }
}


fn extract_key_and_value(compiler_option: &String) -> Result<(String, String), &'static str> {
    if compiler_option.len() > 0 {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"/^-{1,2}((?P<key_with_value>[[:alpha:]]+)=(?P<value_with_key>.+))|(?P<single_key>[[:alpha:]]+)$").unwrap();
        }
                
        let key_capture = RE.captures(compiler_option).and_then(|cap| {cap.name("key_with_value").map(|key| key.as_str())});
        
        match key_capture {
            Some(key) => {
                // case "key_with_value" - value expected
                let value_capture = RE.captures(compiler_option).and_then(|cap| {cap.name("value_with_key").map(|key| key.as_str())});

                match value_capture {
                    Some(value) => {
                        return Ok((key.to_string(), value.to_string()))
                    }

                    None => {
                        //let mut error_message = format!("Compiler option value not found in option: {}", compiler_option);
                        //return Err(error_message.as_ref())
                        return Err("Compiler option value not found")
                    }
                }
            }
            None => {
                // case "single_key" - no value expected
                let key_capture = RE.captures(compiler_option).and_then(|cap| {cap.name("single_key").map(|key| key.as_str())});
                let value = String::new();

                match key_capture {
                    Some(key) => {
                        return Ok((key.to_string(), value))
                    }

                    None => {
                        //let mut error_message = format!("Compiler option key not found in option: {}", compiler_option);
                        //return Err(&error_message.as_ref())
                        return Err("Compiler option key not found")
                    }
                }

            }
        }      
    }    
    else {
        Err("Option string is empty")
    }
}



