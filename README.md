## Covid-19 contract tracing simulator

A toy project to simulate transmission of covid-19 under varyous types of
contract tracing.

*Disclaimer* I'm not an epidemiologist, I have no expertise in the virus. This
project really is just a toy and a learning tool for me.


### Running

``` shell
nix-shell
cargo run --bin covid-tracing-sim
```

### Configuration

Initial state is defined in `config/state.dhall`. Simulation parameters are
defined in `config/params.dhall`. Types are defined in
`config/util/types.dhall`. See [dhall-lang.org](https://dhall-lang.org) for more
information about the dhall configuration language.

The simulator uses `dhall-to-json` internally to convert the configuration files
into JSON, then it deserializes those into rust values with
[serde](https://serde.rs). Rust and serde are both typed, so there is quite a
lot of repetition between definition of the dhall types and rust types. It would
be cool if dhall could be used as an IDL within rust as described so that we
only have to define the types once.

### State of the project

Running currently dumps successive states to stdout

 - [x] Define world state
 - [x] Simulation moves people around
 - [-] Simulation transmits infections from person to person
 - [ ] World state includes covid-testing
 - [ ] World state includes contact tracing data
 - [ ] Visualzation of state
 - [ ] Plotting statistics
