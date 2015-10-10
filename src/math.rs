///
///  jmath.rs
///
///  Created by Mitchell Nordine at 04:23AM on May 30, 2014.
///
///

use num::{Float, NumCast};
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
pub fn lerp<F: Float>(start: F, stop: F, amt: F) -> F {
    start + (stop - start) * amt
}

/// Map a value from a given range to a new given range.
#[inline]
pub fn map_range<X, Y>(val: X, in_min: X, in_max: X, out_min: Y, out_max: Y) -> Y where
    X: NumCast,
    Y: NumCast + Copy,
{
    use epsilon::epsilon;

    let val_f: f64 = NumCast::from(val).unwrap();
    let in_min_f: f64 = NumCast::from(in_min).unwrap();
    let in_max_f: f64 = NumCast::from(in_max).unwrap();
    let out_min_f: f64 = NumCast::from(out_min).unwrap();
    let out_max_f: f64 = NumCast::from(out_max).unwrap();

    if (in_min_f - in_max_f).abs() < epsilon() {
        println!("utils::math::map_range warning: avoiding possible divide by zero, \
                 in_min ({}) and in_max({})", in_min_f, in_max_f);
        return out_min;
    }
    Y::from((val_f - in_min_f) / (in_max_f - in_min_f) * (out_max_f - out_min_f) + out_min_f)
        .unwrap()
}

/// Models the CPP remainder function.
#[inline]
pub fn remainder<F: Float>(numer: F, denom: F) -> F {
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
pub fn wrap<F: Float>(val: F, mut from: F, mut to: F) -> F {
    if from > to { mem::swap(&mut from, &mut to); }
    let cycle = to - from;
    if cycle == F::zero() { return to; }
    val - cycle * ((val - from) / cycle).floor()
}

/// The logistic aka sigmoid function.
#[inline]
pub fn sigmoid<F: Float>(f: F) -> F {
    use std::f64::consts::E;
    let e = F::from(E).unwrap();
    F::one() / (F::one() + e.powf(-f))
}

