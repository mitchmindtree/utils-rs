//!
//!  gaussian.rs
//!
//!  Created by Mitchell Nordine at 03:28AM on May 30, 2014.
//!
//!

use math::map_range;
use rand::{Rand, random};
use std::fmt::Debug;
use std::num::{Float, FromPrimitive};

static mut NEXT_VALUE: Option<f64> = None;

#[inline]
fn two<F>() -> F where F: Float { let one: F = Float::one(); one + one }

/// Gen raw gaussian value with dist. at 0.
#[inline]
pub fn gen_raw<F>() -> F where F: Float + FromPrimitive + Rand {
    if let Some(next_value) = unsafe { NEXT_VALUE } {
        unsafe { NEXT_VALUE = None; }
        FromPrimitive::from_f64(next_value).unwrap()
    }
    else {
        let (zero, one, two): (F, F, F) = (Float::zero(), Float::one(), two::<F>());
        let (mut va, mut vb, mut s): (F, F, F) = (zero, zero, zero);
        while s >= one || s == zero {
            va = two * random::<F>() - one;
            vb = two * random::<F>() - one;
            s = va * vb + va * vb
        };
        let multi = ((-two) * s.abs().ln() / s).abs().sqrt();
        unsafe { NEXT_VALUE = (vb * multi).to_f64(); }
        va * multi
    }
}

/// Gen gaussian value with dist. at 'n' with rand randomness.
/// Result will always be in range 0.0 - 1.0.
#[inline]
pub fn gen<F>(n: F, randomness: f32) -> F
where F: Float + Rand + FromPrimitive + Debug {
    let (zero, one): (F, F) = (Float::zero(), Float::one());
    assert!(n >= zero && n <= one, "Gaussian::gen : given `n` ({:?}) must \
            be a percentage between 0 and 1.", n);
    let mut ans = gen_raw::<F>()
                * FromPrimitive::from_f32(randomness.powf(2.0)).unwrap()
                + (n * two::<F>() - one);
    ans = map_range(ans, -one, one, zero, one);
    if ans > one || ans < zero {
        gen::<F>(n, randomness)
    } else {
        ans
    }
}

/// Gen gaussian value mapped to a range.
#[inline]
pub fn gen_map<F>(n: F, randomness: f32, min_range: F, max_range: F) -> F
where F: Float + Rand + FromPrimitive + Debug {
    let (zero, one): (F, F) = (Float::zero(), Float::one());
    let perc = map_range(n, min_range, max_range, zero, one);
    map_range(gen(perc, randomness), zero, one, min_range, max_range)
}

// /// A struct used for generating random
// /// values across a Gaussian distribution.
// /// The struct generates two values at a time
// /// and stores the spare for efficiency.
// #[deriving(Clone, Show, Encodable, Decodable)]
// pub struct Gaussian; /*{
//     pub have_next_value: Cell<bool>,
//     pub next_value: Cell<f32>
// }*/
// 
// impl Gaussian {
// 
//     /// Constructor for Gaussian.
//     pub fn new() -> Gaussian {
//         Gaussian /*{
//             have_next_value: Cell::new(false),
//             next_value: Cell::new(0f32)
//         }*/
//     }
// 
//     /// Gen raw gaussian value with dist. at 0.
//     pub fn gen_raw(&self) -> f32 {
//         /*if self.have_next_value.get() {
//             self.have_next_value.set(false);
//             return self.next_value.get();
//         }
//         else {*/
//             let (mut va, mut s) = (0f32, 0f32);
//             while s >= 1.0f32 || s == 0.0f32 {
//                 va = 2.0f32 * random::<f32>() - 1.0f32;
//                 let vb = 2.0f32 * random::<f32>() - 1.0f32;
//                 s = va * vb + va * vb
//             };
//             let multi: f32 = ((-2.0) * s.abs().ln() / s).abs().sqrt();
//             //self.next_value.set(vb * multi);
//             //self.have_next_value.set(true);
//             return va * multi;
//         //}
//     }
// 
//     /// Gen gaussian value with dist. at 'n' with rand randomness.
//     /// Result will always be in range 0.0 - 1.0.
//     pub fn gen(&self, n: f32, rand: f32) -> f32 {
//         assert!(n >= 0.0 && n <= 1.0,
//                 "Gaussian::gen : given `n` ({}) must be a percentage between 0 and 1.",
//                 n);
//         let mut ans = self.gen_raw() * rand.powf(2.0) + (n * 2.0 - 1.0);
//         ans = map_range(ans, -1.0, 1.0, 0f32, 1.0);
//         if ans > 1.0 || ans < 0.0 {
//             self.gen(n, rand)
//         } else {
//             ans
//         }
//     }
// 
//     /// Gen gaussian value mapped to a range.
//     pub fn gen_map<T: Copy + FromPrimitive + ToPrimitive>
//     (&self, n: T, rand: f32, min_range: T, max_range: T) -> T {
//         let perc = map_range(n, min_range, max_range, 0f32, 1f32);
//         map_range(self.gen(perc, rand), 0f32, 1f32, min_range, max_range)
//     }
// 
// }

