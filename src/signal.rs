//!
//!  signal.rs
//!
//!  Created by Mitchell Nordine at 11:48PM on December 06, 2014.
//!
//!

use std::num::{Float, FromPrimitive, ToPrimitive};
use std::rand;
use math;

/// Signal generic struct for simplifying dsp signal generation.
/// Signal should be able to handle any floating point primitive.
#[derive(Copy, Clone, Debug, RustcEncodable, RustcDecodable)]
pub struct Signal<F> {
    /// The main value. If Signal were to be plotted on a cartesian
    /// plane, this value would be 'x' for which we will solve 'y'.
    pub val: F,
    /// Optional: The 'x' value of the second point
    /// (from which we calc our range and gradient)
    x: F,
    /// Optional: The 'y' value of the second point
    /// (from which we calc our range and gradient)
    y: F,
    /// The calculated gradient.
    grad: F,
    /// Depth of the bezier curve.
    pub bez_depth: F,
    /// Frequency of the signal.
    pub freq: F,
    /// Amplitude of the signal.
    pub amp: F
}

/// Times two pi for most methods in 'Signal' implementations.
fn times_two_pi<F>(f: F) -> F where F: Float + FromPrimitive {
    use std::f32::consts::PI_2;
    f * FromPrimitive::from_f32(PI_2).unwrap()
}

/// Get random() mapped from -1.0 to 1.0 for 'Signal::get_noise'.
fn get_rand_signal<F: Float + rand::Rand + FromPrimitive>() -> F {
    let r: F = rand::random();
    r * FromPrimitive::from_f32(2.0).unwrap() - FromPrimitive::from_f32(1.0).unwrap()
}

/// Ported implementation of `_slang_library_noise1()` for our generic noise walk!
#[inline]
pub fn noise_walk<F: Float + FromPrimitive>(phase: F) -> F {
    let uno: F = Float::one();
    let i0: i64 = math::fast_floor(phase);
    let i1: i64 = i0 + 1;
    let x0: F = phase - FromPrimitive::from_i64(i0).unwrap();
    let x1: F = x0 - uno;
    let x12d: F = x1 * x1;
    let x02d: F = x0 * x0;
    let t1: F = uno - x12d;        
    let t0: F = uno - x02d;
    let t0a: F = t0 * t0;
    let g1: f32 = math::grad1(
        math::get_perm_val((i0 & 0xff) as usize) as i64, x0.to_f32().unwrap());
    let n0: F = t0a * t0a * FromPrimitive::from_f32(g1).unwrap(); 
    let t1a: F = t1 * t1;
    let g2: f32 = math::grad1(
        math::get_perm_val((i1 & 0xff) as usize) as i64, x1.to_f32().unwrap());
    let n1: F = t1a * t1a * FromPrimitive::from_f32(g2).unwrap();
    let n0pn1: F = n0 + n1;
    let quarter: F = FromPrimitive::from_f32(0.25f32).unwrap();
    quarter * n0pn1
}

impl<F: Float + rand::Rand + FromPrimitive + ToPrimitive> Signal<F> {

    /// Constructor for Signal
    #[inline]
    pub fn new(val: F) -> Signal<F> {
        Signal {
            val: val,
            x: Float::one(),
            y: Float::zero(),
            grad: Float::zero(),
            bez_depth: Float::zero(),
            freq: Float::one(),
            amp: Float::one(),
        }
    }

    /// Set value for which you will return signal (get_sin/cos/sqr/saw) etc...
    #[inline]
    pub fn set_val(&mut self, val: F) {
        self.val = val;
    }

    /// If you woudl like to return the signal value on a slope, set gradient here.
    #[inline]
    pub fn set_gradient(&mut self, x: F, y: F) {
        self.x = x;
        self.y = y;
        self.grad = x / y;
    }

    /// Set frequency of signal.
    #[inline]
    pub fn set_freq(&mut self, freq: F) {
        self.freq = freq;
    }

    /// Set amplitude of signal.
    #[inline]
    pub fn set_amp(&mut self, amp: F) {
        self.amp = amp;
    }

    /// Set depth of bezier curve. Defaults to 0.
    #[inline]
    pub fn set_bez_depth(&mut self, bez_depth: F) {
        self.bez_depth = bez_depth;
    }

    /// Helper function for 'get_bezier'.
    #[inline]
    fn get_bezier_pt(na: F, nb: F, perc: F) -> F {
        let diff: F = nb - na;
        diff * perc + na
    }

    /// Helper function for 'get_bezier'.
    #[inline]
    fn get_y1(y: F, one: F) -> F {
        y / (one + one)
    }

    /// Get signal with bezier curve.
    #[inline]
    pub fn get_bezier(&self) -> F {
        let y1: F = Signal::get_y1(self.y, Float::one());
        let y2: F = y1 + self.bez_depth * y1;
        let relative_val: F = self.val / self.x;
        let ya: F = Signal::get_bezier_pt(Float::zero(), y2, relative_val);
        let yb: F = Signal::get_bezier_pt(y2, self.y, relative_val);
        Signal::get_bezier_pt(ya, yb, relative_val)
    }

    /// Get oscillator with amplitude and bezier.
    #[inline]
    pub fn get_result(&self, val: F) -> F {
        self.amp * val + self.get_bezier()
    }

    /// Get sine wave signal result at val.
    #[inline]
    pub fn get_sin(&self) -> F {
        self.get_result((times_two_pi(self.val) * self.freq / self.x).sin())
    }

    /// Get cosine wave signal result at val.
    #[inline]
    pub fn get_cos(&self) -> F {
        self.get_result((times_two_pi(self.val) * self.freq / self.x).cos())
    }

    /// Get saw wave signal result at val.
    #[inline]
    pub fn get_saw(&self) -> F {
        self.get_result((math::fmod((self.val * self.freq / self.x), Float::one())) * FromPrimitive::from_int(-2).unwrap() + FromPrimitive::from_int(1).unwrap())
    }
    
    /// Get square wave signal result at val.
    #[inline]
    pub fn get_sqr(&self) -> F {
        self.get_result((times_two_pi(self.val) * self.freq / self.x).sin().signum())
    }

    /// Get noise signal result at val.
    #[inline]
    pub fn get_noise(&self) -> F {
        self.get_result(get_rand_signal())
    }

    /// Ported implementation of `_slang_library_noise1()`
    /// for our generic noise walk!
    #[inline]
    pub fn get_noise_walk(&self) -> F {
        noise_walk(self.val)
    }

}

