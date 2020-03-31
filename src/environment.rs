use crate::state::measure;
use crate::state::person::R2;
use crate::state;
use serde::{Deserialize,Serialize};

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub enum Element {
    BoundingBox (Box), // { top: f64, left: f64, width: f64, height: f64 },
    Road(Vec<measure::Location>),
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct Box {
    pub top : f64,
    pub left : f64,
    pub width: f64,
    pub height: f64,
}

/// Naive wall bouncer. Only works if we have 0 or 1 bounding boxes
pub fn bounce_off( r2 : &R2, b : &Box, dt : u32 ) -> R2 {

    // How would oun point change with no walls?
    let dx0 : f64 = r2.velocity * r2.heading.cos() * (dt as f64);
    let dy0 : f64 = r2.velocity * r2.heading.sin() * (dt as f64);

    let (new_x, x_flips) =
        if r2.location.x + dx0 < b.left {
            (b.left + (r2.location.x + dx0 - b.left), true)
        } else if r2.location.x + dx0 > b.left + b.width {
            (b.left + b.width - (r2.location.x + dx0 - (b.left + b.width)), true)
        } else {
          (r2.location.x + dx0, false)
        };

    let (new_y, y_flips) =
        if r2.location.y + dy0 < b.top {
            (b.top + (r2.location.y + dy0 - b.top), true)
        } else if r2.location.y + dy0 > b.top + b.height {
            (b.top + b.height - (r2.location.y + dy0 - (b.top + b.height)), true)
        } else {
          (r2.location.y + dy0, false)
        };

    let new_heading = {
        let h0 = r2.heading;
        if x_flips && y_flips {
            h0 + std::f64::consts::PI
        } else if x_flips {
            let (x,y) = (h0.cos(), h0.sin());
            y.atan2(-x)
        } else if y_flips {
            let (x,y) = (h0.cos(), h0.sin());
            (-y).atan2(x)
        } else {
            h0
        }};
   
    R2
        { location: state::measure::Location { x: new_x, y: new_y }
        , heading:  new_heading
        , velocity: r2.velocity
        }
}
