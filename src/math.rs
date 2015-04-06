///
///  jmath.rs
///
///  Created by Mitchell Nordine at 04:23AM on May 30, 2014.
///
///

use num::Float;
use num::PrimInt as Int;
use std::mem;
use std::num::{FromPrimitive, ToPrimitive};

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

/// Implementation of grad1 for the ported _slang_library_noise1 method
#[inline]
pub fn grad1(hash: i64, x: f32) -> f32 {
    let h: i64 = hash & 15;
    let mut grad: f32 = 1.0f32 + ((h & 7) as f32);
    if h & 8 > 0 { grad = (-1.0f32) * grad; }
    grad * x
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

/// Implementation of perm for the ported _slang_library_noise1 method
const PERM : [u8; 512] = [151u8, 160u8, 137u8, 91u8, 90u8, 15u8,
      131u8, 13u8, 201u8,95u8,96u8,53u8,194u8,233u8,7u8,225u8,140u8,36u8,103u8,30u8,69u8,142u8,8u8,99u8,37u8,240u8,21u8,10u8,23u8,
      190u8, 6u8,148u8,247u8,120u8,234u8,75u8,0u8,26u8,197u8,62u8,94u8,252u8,219u8,203u8,117u8,35u8,11u8,32u8,57u8,177u8,33u8,
      88u8,237u8,149u8,56u8,87u8,174u8,20u8,125u8,136u8,171u8,168u8, 68u8,175u8,74u8,165u8,71u8,134u8,139u8,48u8,27u8,166u8,
      77u8,146u8,158u8,231u8,83u8,111u8,229u8,122u8,60u8,211u8,133u8,230u8,220u8,105u8,92u8,41u8,55u8,46u8,245u8,40u8,244u8,
      102u8,143u8,54u8, 65u8,25u8,63u8,161u8, 1u8,216u8,80u8,73u8,209u8,76u8,132u8,187u8,208u8, 89u8,18u8,169u8,200u8,196u8,
      135u8,130u8,116u8,188u8,159u8,86u8,164u8,100u8,109u8,198u8,173u8,186u8, 3u8,64u8,52u8,217u8,226u8,250u8,124u8,123u8,
      5u8,202u8,38u8,147u8,118u8,126u8,255u8,82u8,85u8,212u8,207u8,206u8,59u8,227u8,47u8,16u8,58u8,17u8,182u8,189u8,28u8,42u8,
      223u8,183u8,170u8,213u8,119u8,248u8,152u8, 2u8,44u8,154u8,163u8, 70u8,221u8,153u8,101u8,155u8,167u8, 43u8,172u8,9u8,
      129u8,22u8,39u8,253u8, 19u8,98u8,108u8,110u8,79u8,113u8,224u8,232u8,178u8,185u8, 112u8,104u8,218u8,246u8,97u8,228u8,
      251u8,34u8,242u8,193u8,238u8,210u8,144u8,12u8,191u8,179u8,162u8,241u8, 81u8,51u8,145u8,235u8,249u8,14u8,239u8,107u8,
      49u8,192u8,214u8, 31u8,181u8,199u8,106u8,157u8,184u8, 84u8,204u8,176u8,115u8,121u8,50u8,45u8,127u8, 4u8,150u8,254u8,
      138u8,236u8,205u8,93u8,222u8,114u8,67u8,29u8,24u8,72u8,243u8,141u8,128u8,195u8,78u8,66u8,215u8,61u8,156u8,180u8,
      151u8,160u8,137u8,91u8,90u8,15u8,
      131u8,13u8,201u8,95u8,96u8,53u8,194u8,233u8,7u8,225u8,140u8,36u8,103u8,30u8,69u8,142u8,8u8,99u8,37u8,240u8,21u8,10u8,23u8,
      190u8, 6u8,148u8,247u8,120u8,234u8,75u8,0u8,26u8,197u8,62u8,94u8,252u8,219u8,203u8,117u8,35u8,11u8,32u8,57u8,177u8,33u8,
      88u8,237u8,149u8,56u8,87u8,174u8,20u8,125u8,136u8,171u8,168u8, 68u8,175u8,74u8,165u8,71u8,134u8,139u8,48u8,27u8,166u8,
      77u8,146u8,158u8,231u8,83u8,111u8,229u8,122u8,60u8,211u8,133u8,230u8,220u8,105u8,92u8,41u8,55u8,46u8,245u8,40u8,244u8,
      102u8,143u8,54u8, 65u8,25u8,63u8,161u8, 1u8,216u8,80u8,73u8,209u8,76u8,132u8,187u8,208u8, 89u8,18u8,169u8,200u8,196u8,
      135u8,130u8,116u8,188u8,159u8,86u8,164u8,100u8,109u8,198u8,173u8,186u8, 3u8,64u8,52u8,217u8,226u8,250u8,124u8,123u8,
      5u8,202u8,38u8,147u8,118u8,126u8,255u8,82u8,85u8,212u8,207u8,206u8,59u8,227u8,47u8,16u8,58u8,17u8,182u8,189u8,28u8,42u8,
      223u8,183u8,170u8,213u8,119u8,248u8,152u8, 2u8,44u8,154u8,163u8, 70u8,221u8,153u8,101u8,155u8,167u8, 43u8,172u8,9u8,
      129u8,22u8,39u8,253u8, 19u8,98u8,108u8,110u8,79u8,113u8,224u8,232u8,178u8,185u8, 112u8,104u8,218u8,246u8,97u8,228u8,
      251u8,34u8,242u8,193u8,238u8,210u8,144u8,12u8,191u8,179u8,162u8,241u8, 81u8,51u8,145u8,235u8,249u8,14u8,239u8,107u8,
      49u8,192u8,214u8, 31u8,181u8,199u8,106u8,157u8,184u8, 84u8,204u8,176u8,115u8,121u8,50u8,45u8,127u8, 4u8,150u8,254u8,
      138u8,236u8,205u8,93u8,222u8,114u8,67u8,29u8,24u8,72u8,243u8,141u8,128u8,195u8,78u8,66u8,215u8,61u8,156u8,180u8];

/// Implementation of perm lookup for the ported _slang_library_noise1 method
pub fn get_perm_val(i: usize) -> u8 { PERM[i] }

