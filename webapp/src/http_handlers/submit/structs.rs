#![allow(unused)]
use rocket::serde::{Serialize, Deserialize};
use compiler::data::output_data;

pub mod submit_input;
pub mod submit_output;
pub mod submit_headers;
pub mod runner_output;

pub use submit_input::SubmitInput;
pub use submit_output::SubmitOutput;
pub use submit_headers::SubmitHeaders;
pub use runner_output::RunnerOutput;
