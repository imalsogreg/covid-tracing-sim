use std::collections::BTreeMap;
use std::process;
use serde_json;
use std::io::Read;
use std::io::Write;

use super::state;
use super::params;


/// Load initial rust state from a dhall expression
pub fn load_state(s : &str) -> Result<state::State, serde_json::Error> {
    load_dhall_expr::<state::State>(s)
}

/// Load simulation parameters from a dhall expression
pub fn load_params(s : &str) -> Result<params::Params, serde_json::Error> {
    load_dhall_expr::<params::Params>(s)
}


pub fn load_dhall_expr<T>(s : &str) -> Result<T, serde_json::Error>
where T: serde::de::DeserializeOwned
{
    let mut dhall_process =
        process::Command::new("dhall-to-json")
        .stdin(process::Stdio::piped())
        .stdout(process::Stdio::piped())
        .spawn()
        .expect("Could not spawn dhall-to-json");
    dhall_process.stdin.as_mut().unwrap().write_all(s.as_bytes()).expect("failed write");
    let output = dhall_process.wait_with_output().expect("failed wait");
    let data = std::str::from_utf8( output.stdout.as_slice() ).expect("conv");

    serde_json::from_str(data)
}
