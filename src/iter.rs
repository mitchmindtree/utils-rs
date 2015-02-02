//!
//!  iter.rs
//!
//!  Created by Mitchell Nordine at 11:27AM on November 07, 2014.
//!
//!  A collection of custom iterators for iterating things!
//!

pub use self::prev_iter::ZipPrev;
pub use self::sample_on::SampleOn;

pub mod prev_iter {

    /// A trait that produces the previous step's element along
    /// with the current step.
    pub trait ZipPrev: Iterator + Sized {
        fn zip_prev(self) -> Items<<Self as Iterator>::Item, Self> {
            Items {
                maybe_prev: None,
                iter: self,
            }
        }
    }

    /// A struct produced by the ZipPrev iterator.
    pub struct Items<A, I> {
        maybe_prev: Option<A>,
        iter: I,
    }

    impl<I> Iterator for Items<<I as Iterator>::Item, I>
    where I: Iterator, <I as Iterator>::Item: Clone {
        type Item = (<I as Iterator>::Item, Option<<I as Iterator>::Item>);
        #[inline]
        fn next(&mut self) -> Option<<Self as Iterator>::Item> {
            let Items { ref mut iter, ref mut maybe_prev } = *self;
            if let Some(item) = iter.next() {
                let maybe_prev_clone = maybe_prev.clone();
                *maybe_prev = Some(item.clone());
                Some((item, maybe_prev_clone))
            } else {
                None
            }
        }
    }

    impl<I> ZipPrev for I
    where I: Iterator,
          <I as Iterator>::Item: Clone {}

}

pub mod sample_on {

    /// Sample from the current iterator every time an iteration occurs on another iterator.
    pub trait SampleOn: Sized + Iterator {
        #[inline]
        fn sample_on<O: Iterator>(self, other: O) -> Items<Self, O> {
            Items { sample: self, on: other, last_sample: None, is_infinite: false }
        }
    }

    /// Iterator returned from the `sample_on` method.
    pub struct Items<A, B> where A: Iterator, B: Iterator {
        sample: A,
        on: B,
        last_sample: Option<<A as Iterator>::Item>,
        is_infinite: bool,
    }

    impl<A, B> Items<A, B> where A: Iterator, B: Iterator {
        /// Construct a never-ending signal.
        #[inline]
        pub fn infinite(self) -> Items<A, B> {
            Items { is_infinite: true, ..self }
        }
    }

    impl<A, B> Iterator for Items<A, B>
    where A: Iterator, B: Iterator, <A as Iterator>::Item: Clone {
        type Item = <A as Iterator>::Item;
        fn next(&mut self) -> Option<<A as Iterator>::Item> {
            while let None = self.on.next() {}
            match self.sample.next() {
                None => if self.is_infinite { self.last_sample.clone() } else { None },
                Some(sample) => {
                    self.last_sample = Some(sample.clone());
                    Some(sample)
                },
            }
        }
    }

}
