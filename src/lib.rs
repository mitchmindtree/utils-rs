//!
//!  mod.rs
//!
//!  Created by Mitchell Nordine at 11:25AM on November 07, 2014.
//!
//!

extern crate num;
extern crate rand;
extern crate rustc_serialize;
extern crate time;

pub use epsilon::{
    Epsilon,
    epsilon,
};
pub use factorisation::{
    is_factor,
    are_any_factors,
    are_all_factors,
    is_prime,
    lowest_non_one,
    get_all_factors,
};
pub use fps::{
    Fps,
};
pub use iter::SampleOn;
pub use math::{
    clamp,
    fmod,
    in_range,
    lerp,
    map_range,
    modulo,
    remainder,
    wrap,
};
pub use noise_walk::noise_walk;
pub use signal::{
    Signal,
};
pub use vec::{
    TakeOnly,
};

mod chain;
pub mod epsilon;
pub mod factorisation;
pub mod fps;
pub mod iter;
pub mod math;
mod noise_walk;
pub mod signal;
pub mod vec;

