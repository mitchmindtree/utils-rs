//!
//!  mod.rs
//!
//!  Created by Mitchell Nordine at 11:25AM on November 07, 2014.
//!
//!

#![feature(if_let)]

extern crate serialize;

pub use factorisation::{
    is_factor,
    are_any_factors,
    are_all_factors,
    is_prime,
    lowest_non_one,
    get_all_factors,
};
pub use gaussian::{
    gen,
    gen_map,
    gen_raw,
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
pub use vec::{
    TakeOnly,
};

pub mod factorisation;
pub mod gaussian;
pub mod iter;
pub mod math;
pub mod vec;

