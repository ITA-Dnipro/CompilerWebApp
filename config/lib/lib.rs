pub mod configurable;

pub struct Config
{
    vars: Vec<u128>
}

impl Config 
{
    fn new() -> Self
    {
        Config { vars: vec![] }
    }
}

mod tests
{
    use crate::Config;
    #[test]
    fn casual() 
    {
        let config = Config::new();
    }
}