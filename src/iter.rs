//!
//!  iter.rs
//!
//!  Created by Mitchell Nordine at 11:27AM on November 07, 2014.
//!
//!  A collection of custom iterators for iterating things!
//!

pub use self::prev_iter::ZipPrev;

pub mod prev_iter {

    /// A trait that produces the previous step's element along
    /// with the current step.
    pub trait ZipPrev<A, I>: Iterator<A>
    where A: Clone, I: Iterator<A> {
        fn zip_prev(self) -> PairIterator<A, I>;
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
            let PairIterator { ref mut iter, ref mut maybe_prev } = *self;
            if let Some(item) = iter.next() {
                let maybe_prev_clone = maybe_prev.clone();
                *maybe_prev = Some(item.clone());
                Some((item, maybe_prev_clone))
            } else {
                None
            }
        }
    }

    impl<A, I> ZipPrev<A, I> for I
    where A: Clone, I: Iterator<A> {
        fn zip_prev(self) -> PairIterator<A, I> {
            PairIterator {
                iter: self,
                maybe_prev: None,
            }
        }
    }

}

