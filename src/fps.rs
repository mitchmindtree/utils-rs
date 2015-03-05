//!
//!  fps.rs
//!
//!  Created by Mitchell Nordine at 08:31PM on February 02, 2015.
//!
//!

use std::old_io::timer::sleep;
use std::time::duration::Duration;
use time;

/// A signal that returns delta time at a rate so that
/// there are `fps` frames per seconds.
#[derive(Copy)]
pub struct Fps {
    fps: f64,
    last_ns: u64,
    high_priority: bool,
}

impl Fps {

    /// Construct a new Fps struct with a maximum `frames_per_sec`.
    pub fn new(fps: f64) -> Fps {
        assert!(fps > 0.0, "The given frames per seconds must be greater than 0.");
        Fps { fps: fps, last_ns: time::precise_time_ns(), high_priority: false }
    }

    /// A builder method for constructing a high priority Fps signal.
    /// This will consume more cpu but will result in a far more accurate frame rate.
    pub fn high_priority(self) -> Fps {
        Fps { high_priority: true, ..self }
    }

    /// Return the time since the last frame.
    #[inline]
    pub fn frame_ns(&self) -> u64 { (BILLION / self.fps) as u64 }

    /// Return the dt in nanoseconds for the given t in nanoseconds
    #[inline]
    pub fn get_dt_ns(&self, t: u64) -> u64 {
        if t >= self.last_ns {
            t - self.last_ns
        } else {
            use std::u64::MAX;
            (MAX - self.last_ns) + t
        }
    }

}

const BILLION: f64 = 1_000_000_000.0;
pub type DeltaSecs = f64;

/// Convert nanoseconds to seconds.
#[inline]
fn ns_to_secs(n: u64) -> f64 { n as f64 / BILLION }

impl Iterator for Fps {
    type Item = DeltaSecs;
    fn next(&mut self) -> Option<DeltaSecs> {
        let frame_ns = self.frame_ns();
        let t_ns = time::precise_time_ns();
        let dt_ns = self.get_dt_ns(t_ns);
        if dt_ns >= frame_ns {
            self.last_ns = t_ns;
            Some(ns_to_secs(dt_ns))
        }
        else {
            if !self.high_priority {
                sleep(Duration::nanoseconds((frame_ns - dt_ns) as i64));
            }
            let mut t_ns = time::precise_time_ns();
            let mut dt_ns = self.get_dt_ns(t_ns);
            while dt_ns < frame_ns {
                t_ns = time::precise_time_ns();
                dt_ns = self.get_dt_ns(t_ns);
            }
            self.last_ns = t_ns;
            Some(ns_to_secs(dt_ns))
        }
    }
}

