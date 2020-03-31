use chrono::prelude::*;

pub mod measure {

    #[derive(Debug, PartialEq, serde::Deserialize, serde::Serialize)]
    pub struct Location {
        pub x : f64,
        pub y : f64,
    }

}


pub mod health {

    // TODO: Lookup preexisting conditions from CDC
    #[derive(Debug, PartialEq, serde::Deserialize, serde::Serialize)]
    pub enum Condition {
        Asthma,
        Diabetes,
        Obeisity,
    }

    #[derive(Debug, PartialEq, serde::Deserialize, serde::Serialize)]
    pub struct TimedInfection {
        pub time : chrono::NaiveDateTime,
        pub exposure : ExposureLevel,
    }

    #[derive(Debug, PartialEq, serde::Deserialize, serde::Serialize)]
    pub enum ExposureLevel {
        Low,
        Medium,
        High
    }

    #[derive(Debug, PartialEq, serde::Deserialize, serde::Serialize)]
    pub struct Health {
        pub birthdate  : chrono::NaiveDate,
        pub conditions : Vec<Condition>,
        pub infections : Vec<TimedInfection>,
    }
}

pub mod person {

    #[derive(Debug, PartialEq, serde::Deserialize, serde::Serialize)]
    pub struct ID (pub i32);

    #[derive(Debug, PartialEq, serde::Deserialize, serde::Serialize)]
    pub struct R2 {
        pub location : crate::state::measure::Location,
        pub velocity : f64,
        pub heading  : f64,
    }

    #[derive(Debug, PartialEq, serde::Deserialize, serde::Serialize)]
    pub struct Person {
        pub id     : ID,
        pub r2     : R2,
        pub health : crate::state::health::Health,
    }
}

/// Saveable and loadable world state
#[derive(Debug, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct State
  { pub people : Vec<person::Person>
  , pub now    : NaiveDateTime
  }

/// Run-time extra state
#[derive(Debug, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct RuntimeState
  { pub peopleTree : u32 }
