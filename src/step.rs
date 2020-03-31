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

    let interpersonal_distance_squared = |(me, other_person) : (&Person,&Person)| {
        (me.r2.location.x - other_person.r2.location.x).powi(2) +
            (me.r2.location.y - other_person.r2.location.y).powi(2)
    };

    let is_infectious = |other_person : &Person| {
        !other_person.health.infections.is_empty()
    };

    // Make this more interesting later
    let infectivity = |other_person : &Person| {
        if is_infectious(other_person) {1.0} else {0.0}
    };

    let infection_receptivity = |me : &Person|{
        if me.health.infections.is_empty() {
            let age = now - me.health.birthdate.and_hms(0,0,0);
            (age.num_days() as f64 / 365.0 / 200.0 + 0.2 * me.health.conditions.len() as f64).max(1.0)
        } else {
            // Already got it. Immune!
            0.0
        }
    };

    let infection_per_second = |(me, other_person) : (&Person, &Person)| {
      infection_receptivity(me)
            * infectivity(other_person)
            * (1.0 / interpersonal_distance_squared((me,other_person))).max(1.0)
    };

    for other_person in people {
        if is_infectious(other_person) && infection_per_second((&p, other_person)) > 0.1 {
                p.health.infections.push( TimedInfection { time: now, exposure: ExposureLevel::Medium });
        }
    }
    p

}
