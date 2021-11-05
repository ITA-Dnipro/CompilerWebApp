use super::lang_info::LangInfo;

pub struct FlagsValidator
{}

impl FlagsValidator
{
    // Validates that given flags suit the given language
    pub fn validate(flags: &str, lang_info: &LangInfo) -> bool
    {
        let parsed_flags = lang_info.parser.parse(&flags);

        if lang_info.are_flags_whitelist
        {
            lang_info.compiler_flags.is_superset(&parsed_flags)
        }
        else
        {
            // Comparing manually because HashSet.intersect() doesn't see Undefined and UndefinedFlag(val) as the same
            // Nor it should
            let mut valid: bool = true;

            for flag in &parsed_flags
            {
                for lang_flag in &lang_info.compiler_flags
                {
                    if *lang_flag == *flag
                    {
                        valid = false;
                        break;
                    }
                }
                
            }

            valid
        }      
    }
}
