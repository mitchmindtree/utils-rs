///
///  jmath.rs
///
///  Created by Mitchell Nordine at 04:23AM on May 30, 2014.
///
///

use num::{Float, FromPrimitive, ToPrimitive};
use num::PrimInt as Int;
use std::mem;

/// Clamp a value to a range.
#[inline]
pub fn clamp<T: PartialOrd>(val: T, min: T, max: T) -> T {
    if val < min { min } else { if val > max { max } else { val } }
}

/// Models the CPP fmod function.
#[inline]
pub fn fmod<F: Float>(numer: F, denom: F) -> F {
    let rquot: F = (numer / denom).floor();
    numer - rquot * denom
}

/// Check if value is in range.
#[inline]
pub fn in_range<T: Ord>(val: T, min: T, max: T) -> bool {
    val >= min && val <= max
}

/// Interpolate from start to stop 'amt' amount.
#[inline]
pub fn lerp(start: f32, stop: f32, amt: f32) -> f32 {
    start + (stop - start) * amt
}

/// Map a value from a given range to a new given range.
#[inline]
pub fn map_range<X: Copy + FromPrimitive + ToPrimitive,
                 Y: Copy + FromPrimitive + ToPrimitive>
    (val: X, in_min: X, in_max: X, out_min: Y, out_max: Y) -> Y {
    use epsilon::epsilon;
    let (val_f, in_min_f, in_max_f, out_min_f, out_max_f) = (
        val.to_f64().unwrap(),
        in_min.to_f64().unwrap(),
        in_max.to_f64().unwrap(),
        out_min.to_f64().unwrap(),
        out_max.to_f64().unwrap(),
    );
    if (in_min_f - in_max_f).abs() < epsilon() {
        println!("jmath Warning: map(): avoiding possible divide by zero, \
                 in_min ({}) and in_max({})", in_min_f, in_max_f);
        return out_min;
    }
    FromPrimitive::from_f64(
        (val_f - in_min_f) / (in_max_f - in_min_f) * (out_max_f - out_min_f) + out_min_f
    ).unwrap()
}

/// Models the CPP remainder function.
#[inline]
pub fn remainder<F: Float + FromPrimitive + ToPrimitive>(numer: F, denom: F) -> F {
    let rquot: F = (numer / denom).round();
    numer - rquot * denom
}

/// The modulo function.
#[inline]
pub fn modulo<I: Int>(a: I, b: I) -> I {
    match a % b {
        r if (r > I::zero() && b < I::zero())
          || (r < I::zero() && b > I::zero()) => (r + b),
        r                                         => r,
    }
}

/// Wrap value to a range.
#[inline]
pub fn wrap(val: f32, mut from: f32, mut to: f32) -> f32 {
    if from > to { mem::swap(&mut from, &mut to); }
    let cycle = to - from;
    if cycle == 0.0 { return to; }
    val - cycle * ((val - from) / cycle).floor()
}

