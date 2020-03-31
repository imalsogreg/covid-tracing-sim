use std::collections::BTreeMap;
use std::process;
use serde_json;
use std::io::Read;
use std::io::Write;

use super::state;
use super::params;


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

pub fn load_dhall_expr2<T>(s : & str) -> Result<T, serde_json::Error>
    where T: serde::de::DeserializeOwned
{
    println!("spawn");
    let mut dhall_child =
        process::Command::new("dhall-to-json")
        .stdin(process::Stdio::piped())
        .stdout(process::Stdio::piped())
        .spawn()
        .expect("Could not spawn dhall-to-json");

    println!("write_all");
    dhall_child.stdin
               .as_mut()
               .unwrap()
               .write_all(s.as_bytes()).expect("Failed to write s");

    let mut data = String::new();

    println!("read_to_string");
    dhall_child.stdout
               .as_mut()
               .unwrap()
               .read_to_string(&mut data)
               .expect("Failed to read from dhall-to-json");

    println!("serde_json");
    serde_json::from_str(data.as_str())
}

pub fn load_state(s : &str) -> Result<state::State, serde_json::Error> {
    load_dhall_expr::<state::State>(s)
}

pub fn load_params(s : &str) -> Result<params::Params, serde_json::Error> {
    load_dhall_expr::<params::Params>(s)
}


// pub fn load_dhall_state(f : &Path) -> Result<state::State, serde_json::Error> {
//     let cat_child =
//         process::Command::new("cat")
//         .arg(f.to_str().expect("Could not convert path"))
//         .stdout(process::Stdio::piped())
//         .spawn()
//         .expect("Failed to run cat");

//     let cat_out = cat_child.stdout.expect("Failed to open cat stdout");

//     let dhall_result =
//         process::Command::new("dhall-to-json")
//         .stdin(process::Stdio::from(cat_out))
//         .stderr(process::Stdio::piped())
//         .output()
//         .expect("Could not spawn dhall-to-json");

//   let data : &str = std::str::from_utf8(dhall_result.stdout.as_slice()).expect("got str");
//   serde_json::from_str(data)
// }
