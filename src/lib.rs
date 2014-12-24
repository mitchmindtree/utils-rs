//!
//!  mod.rs
//!
//!  Created by Mitchell Nordine at 11:25AM on November 07, 2014.
//!
//!

extern crate "rustc-serialize" as rustc_serialize;

pub use envelope::Envelope;
pub use envelope::Point as EnvPoint;
pub use factorisation::{
    is_factor,
    are_any_factors,
    are_all_factors,
    is_prime,
    lowest_non_one,
    get_all_factors,
};
pub use iter::{
    ZipPrev,
};
pub use math::{
    clamp,
    fast_floor,
    fmod,
    grad1,
    in_range,
    lerp,
    map_range,
    modulo,
    remainder,
    wrap,
};
pub use signal::{
    noise_walk,
    Signal,
};
pub use vec::{
    TakeOnly,
};

pub mod envelope;
pub mod factorisation;
pub mod gaussian;
pub mod iter;
pub mod math;
pub mod signal;
pub mod vec;

