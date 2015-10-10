

/// A macro used to simplify chaining together multiple `IntoIter` types.
///
/// Returns a single iterator yielding all of the chained elements.
#[macro_export]
macro_rules! chain {
    () => { ::std::iter::empty() };
    ($first: expr) => { $first.into_iter() };
    ($first: expr, $($iter:expr),*) => { $first.into_iter()$(.chain($iter))* };
}



