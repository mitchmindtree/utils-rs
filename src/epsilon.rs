//!
//!  epsilon.rs
//!
//!  Created by Mitchell Nordine at 03:46PM on February 02, 2015.
//!
//!


/// A trait for finding the smallest fraction possible with a given Float type.
pub trait Epsilon {
    /// Return the difference between 1 and the next lowest value that follows 1.
    fn epsilon() -> Self;
}

impl Epsilon for f32 {
    #[inline]
    fn epsilon() -> f32 { ::std::f32::EPSILON }
}

impl Epsilon for f64 {
    #[inline]
    fn epsilon() -> f64 { ::std::f64::EPSILON }
}

/// A helper function for bypassing the Epsilon trait's namespace where
/// the Epsilon type can be inferred.
#[inline]
pub fn epsilon<F>() -> F where F: Epsilon { Epsilon::epsilon() }
