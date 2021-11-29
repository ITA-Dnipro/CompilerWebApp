#![allow(unused)]
use rocket::serde::{Serialize, Deserialize};
use compiler::data::output_data;

/// POST /submit request body struct
pub mod submit_input;
/// POST /submit response body struct
pub mod submit_output;
/// POST /submit request headers struct
pub mod submit_headers;
/// runner results struct
pub mod runner_output;

pub use submit_input::SubmitInput;
pub use submit_output::SubmitOutput;
pub use submit_headers::SubmitHeaders;
pub use runner_output::RunnerOutput;
