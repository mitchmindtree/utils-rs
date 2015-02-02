//!
//!  vec.rs
//!
//!  Created by Mitchell Nordine at 01:25AM on November 18, 2014.
//!
//!


/// Take only the elements that return `true` from the given condition.
pub trait TakeOnly<T> {
    /// Take only the elements that return `true` from the given condition.
    fn take_only<F>(&mut self, F) -> Vec<T> where F: Fn(&T) -> bool;
}

impl<T> TakeOnly<T> for Vec<T> {
    fn take_only<F>(&mut self, cond: F) -> Vec<T> where F: Fn(&T) -> bool {
        let mut vec = Vec::with_capacity(self.len());
        let mut i = 0;
        while i < self.len() {
            let cond_result = cond(&self[i]);
            if cond_result {
                vec.push(self.remove(i));
            } else {
                i += 1;
            }
        }
        vec
    }
}

