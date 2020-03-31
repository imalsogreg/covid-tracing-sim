mod state;
mod config;
mod params;
mod environment;
mod step;

use serde_json;

fn main() {
    let mut state0 = config::load_state("./config/state.dhall").expect("state load failed");
    let params = config::load_params("./config/params.dhall").expect("params load failed");
    let mut rt_state = crate::state::RuntimeState { peopleTree: 0 };

    let end_time = state0.now + chrono::Duration::seconds(params.simulation_length_seconds as i64);
    while state0.now < end_time {
        let (new_state, new_rt_state) = crate::step::step(state0, rt_state, &params);
        state0 = new_state;
        rt_state = new_rt_state;
        println!("state0: {:?}", state0);
    }
    println!("state_end: {:?}", state0);
}
