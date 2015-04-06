//!
//! factorisation.rs
//!
//! Created by Mitchell Nordine at 05:03AM on May 29, 2014.
//!
//!

use num::PrimInt as Int;
use super::modulo;

/// Check if the queried value is a factor of num.
#[inline]
pub fn is_factor<I>(num: I, query: I) -> bool
where I: Int {
    modulo(num, query) == I::zero()
}

/// Check if any of the queried values are a factor of num.
#[inline]
pub fn are_any_factors<I>(num: I, queries: &[I]) -> bool
where I: Int + Copy {
    queries.iter().any(|query| is_factor(num, *query))
}

/// Check if all of the queried values are a factor of num.
#[inline]
pub fn are_all_factors<I>(num: I, queries: &[I]) -> bool
where I: Int + Copy {
    queries.iter().all(|query| is_factor(num, *query))
}

/// Is the number prime?
#[inline]
pub fn is_prime<I>(num: I) -> bool
where I: Int + Copy {
    match get_all_factors(num).len() {
        1 | 2 => true,
        _ => false
    }
}

/// Return the lowest non-one factor.
#[inline]
pub fn lowest_non_one<I>(n: I) -> Option<I>
where I: Int + Copy {
    let one: I = I::one();
    let mut i: I = one + one;
    while i * i <= n {
        if n % i == I::zero() {
            if i > one { return Some(i) }
            else if i * i != n {
                let n_div_i = n / i;
                if n_div_i > one { return Some(n_div_i) }
            }
        }
        i = i + one;
    }
    None
}

/// Get all factors for 'n'.
#[inline]
pub fn get_all_factors<I>(n: I) -> Vec<I> where I: Int {
    let one: I = I::one();
    let mut factors: Vec<I> = vec![one];
    let mut i: I = one + one;
    while i * i <= n {
        if n % i == I::zero() {
            factors.push(i);
            if i * i != n { factors.push(n / i); }
        }
        i = i + one;
    }
    factors.push(n);
    factors.sort_by(|a, b| a.cmp(b));
    return factors; 
}

