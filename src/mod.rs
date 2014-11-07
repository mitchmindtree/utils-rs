//!
//!  mod.rs
//!
//!  Created by Mitchell Nordine at 11:25AM on November 07, 2014.
//!
//!

pub use iter::{
    ZipPrev,
};
pub use math::{
    modulo,
    map_range,
    clamp,
    in_range,
    lerp,
    wrap,
    fast_floor,
    remainder,
    fmod,
    grad1,
};

pub mod iter;
pub mod linked_list;
pub mod math;

