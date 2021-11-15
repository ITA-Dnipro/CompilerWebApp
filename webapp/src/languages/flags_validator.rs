use super::lang_info::LangInfo;

pub struct FlagsValidator
{
    pub valid: bool,
    pub invalid_flags: Vec<String>
}

impl FlagsValidator
{
    pub fn new() -> FlagsValidator
    {
        FlagsValidator
        {
            valid: true,
            invalid_flags: Vec::<String>::new()
        }
    }

    // Validates that given language supports given flags
    pub fn validate(&mut self, flags: &str, lang_info: &LangInfo) -> bool
    {
        let parsed_flags = lang_info.parser.parse(&flags);

        if lang_info.are_flags_whitelist
        {
            self.valid = lang_info.compiler_flags.is_superset(&parsed_flags);
        }
        else
        {
            // Comparing manually because HashSet.intersect() doesn't see Undefined and UndefinedFlag(val) as the same
            // Nor it should
            for flag in &parsed_flags
            {
                for lang_flag in &lang_info.compiler_flags
                {
                    if *lang_flag == *flag
                    {
                        self.valid = false;
                        self.invalid_flags.push(flag.to_string());
                        break;
                    }
                }            
            }      
        }      

        self.valid
    }
}
