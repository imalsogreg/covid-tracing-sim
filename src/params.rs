use serde::Deserialize;

use crate::environment;

#[derive(Debug, PartialEq, Deserialize)]
pub struct Params {
    pub step_size_seconds : u32,
    pub simulation_length_seconds : u32,
    pub environment : Vec<environment::Element>,
}
