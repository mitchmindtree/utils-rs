//!
//!  iter.rs
//!
//!  Created by Mitchell Nordine at 11:27AM on November 07, 2014.
//!
//!  A collection of custom iterators for iterating things!
//!

pub use self::sample_on::SampleOn;

pub mod sample_on {

    /// Sample from the current iterator every time an iteration occurs on another iterator.
    /// This is primarily used for binding an iterator to another timed iterator. i.e.
    /// `(0..1000).sample_on(Fps::new(60.0))`.
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
        last_sample: Option<A::Item>,
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
        where
            A: Iterator,
            B: Iterator,
            A::Item: Clone
    {
        type Item = A::Item;
        fn next(&mut self) -> Option<A::Item> {
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
