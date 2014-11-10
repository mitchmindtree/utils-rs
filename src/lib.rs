//!
//!  mod.rs
//!
//!  Created by Mitchell Nordine at 11:25AM on November 07, 2014.
//!
//!

#![feature(if_let)]

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

pub mod iter;
pub mod math;

