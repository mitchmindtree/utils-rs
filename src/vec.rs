//!
//!  vec.rs
//!
//!  Created by Mitchell Nordine at 01:25AM on November 18, 2014.
//!
//!


/// Take only the elements that return `true` from the given condition.
pub trait TakeOnly<T> {
    /// Take only the elements that return `true` from the given condition.
    fn take_only(&mut self, |&T| -> bool) -> Vec<T>;
}

impl<T> TakeOnly<T> for Vec<T> {
    fn take_only(&mut self, cond: |&T| -> bool) -> Vec<T> {
        let mut vec = Vec::with_capacity(self.len());
        let mut i = 0;
        while i < self.len() {
            let cond_result = cond(&self[i]);
            if cond_result {
                vec.push(self.remove(i).expect("Tried to take T at `i` - no elem there."));
            } else {
                i += 1;
            }
        }
        vec
    }
}

