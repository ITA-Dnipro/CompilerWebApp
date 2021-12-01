use rocket::{Request, http::Status, request::{self, FromRequest, Outcome}};

/// Flow control relevant request headers
pub struct SubmitHeaders
{
    pub execute: bool
}

impl SubmitHeaders
{
    /// Create a new instance
    pub fn new(execute: bool) -> Self
    {
        Self { execute }
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for SubmitHeaders
{
    type Error = ();

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error>
    {
        let headers = req.headers();
        // Missing or invalid headers are defaulted to false
        if let Some(what) = headers.get_one("execute")
        {
            if let Ok(parsed) = what.parse::<bool>()
            {
                Outcome::Success(Self::new(parsed))
            }
            else 
            {
                Outcome::Success(Self::new(false))
            }
        }
        else 
        {
            Outcome::Success(Self::new(false))
        }
    }
}
