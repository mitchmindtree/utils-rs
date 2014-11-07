//!
//!  iter.rs
//!
//!  Created by Mitchell Nordine at 11:27AM on November 07, 2014.
//!
//!  A collection of custom iterators for iterating things!
//!

pub use prev_iter::ZipPrev;

pub mod prev_iter {

    /// A trait that produces the previous step's element along
    /// with the current step.
    pub trait ZipPrev: Iterator {
        fn zip_prev(self) -> PairIterator;
    }

    /// A struct produced by the ZipPrevious iterator.
    pub struct PairIterator<A, I> {
        maybe_prev: Option<A>,
        iter: I,
    }

    impl<A, I> Iterator<(A, Option<A>)> for PairIterator<A, I>
    where A: Clone, I: Iterator<A> {
        #[inline]
        fn next(&mut self) -> Option<(A, Option<A>)> {
            let PairIterator(ref mut part_iter, ref mut maybe_prev) = *self;
            if let Some(part) = self.part_iter.next() {
                let maybe_prev = self.maybe_prev.clone();
                self.maybe_prev = Some(part);
                Some(part, maybe_prev)
            } else {
                None
            }
        }
    }

    impl<A, I> ZipPrev for I where A: Clone, I: Iterator<A> {
        fn zip_prev(self) -> PairIterator<A, I> {
            PairIterator {
                iter: self,
                maybe_prev: None,
            }
        }
    }

}

