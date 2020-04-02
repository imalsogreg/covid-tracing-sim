use crate::params;
use crate::state;
use crate::state::person::Person;
use crate::state::health::ExposureLevel;
use crate::state::health::TimedInfection;
use crate::environment;

pub fn step
    ( state0        : state::State,
      runtimeState0 : state::RuntimeState,
      params        : &params::Params
    ) -> (state::State, state::RuntimeState)
{

    let mut s = state0;
    let mut r = runtimeState0;

    let new_now = s.now + chrono::Duration::seconds(params.step_size_seconds as i64);

    let new_people = s.people.into_iter().map(|p| {
        let mut new_r2 = p.r2;
        for el in &params.environment {
            match el {
                crate::environment::Element::BoundingBox(bb) => {
                    new_r2 = environment::bounce_off(&new_r2, &bb, params.step_size_seconds);
                },
                crate::environment::Element::Road(_) => {},
            }
        };

        crate::state::person::Person {
            r2: new_r2,
            ..p
        }
    }).collect();


    s = crate::state::State { people: new_people, now: new_now };

    (s,r)
}


fn catch_germs
    ( p0     : state::person::Person,
      people : &Vec<Person>,
      now    : chrono::NaiveDateTime,
      runtimeState : state::RuntimeState
    ) -> state::person::Person
{
    let mut p = p0;

    // This is the lambda parameter of a poisson process representing the
    // events when 'other_person' infects 'me'
    let infection_per_second = |(me, other_person) : (&Person, &Person)| {

      // A peak-infectious person will tend to cause one infection per
      // 20 minutes of exposure with a healthy young person standing 1m
      // away or closer
      //
      // This rate can be modulated up or down by the recipient's health,
      // the distance between the people, etc.
      let baseline_rate = 1.0 / (20.0 * 60.0);

      let interpersonal_distance_squared =
            (me.r2.location.x - other_person.r2.location.x).powi(2) +
            (me.r2.location.y - other_person.r2.location.y).powi(2);

        // TODO make this more interesting - e.g. a person becomes less infective
        // after recovering from their infection
        let other_person_infectivity = if !other_person.health.infections.is_empty() {1.0} else {0.0};

        // My age and my preexisting conditions scale the rate of my infections
        // Every 10 years of age the infection rate doubles
        let my_receptivity = if me.health.infections.is_empty() {
            let my_age_years = (now - me.health.birthdate.and_hms(0,0,0)).num_days() / 365;

            // Every 10 years after age 40 increases my infection rate
            let age_coeff = (2 * (my_age_years - 40) / 10).max(0) + 1;
            let conditions_coeff = (1 + me.health.conditions.len()) as f64;
            age_coeff as f64 + conditions_coeff
        } else {
            // Already got it - immune!
            0.0
        };

        let distance_coeff = (1.0 / interpersonal_distance_squared).max(1.0);

        // Final s^-1 parameter is the baseline rate scaled up or down by infectivity,
        // receptivity, and distance
        baseline_rate * other_person_infectivity * my_receptivity * distance_coeff
    };

    for other_person in people {
        // TODO: This is not the right way to sample a poisson process!
        // But I haven't added any RNG to the simulation yet
        if infection_per_second((&p, other_person)) > 0.1 {
                p.health.infections.push( TimedInfection { time: now, exposure: ExposureLevel::Medium });
        }
    }
    p

}
