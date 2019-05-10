//! # Publishing Crates
//! This crate contains examples of comments used in documenting crates when
//! they are published to Crates.io

/// Adds one to the number given
/// # Examples
/// ```
/// let arg = 5;
/// let answer = publishing::add_one(arg);
/// assert_eq!(6, answer);
/// ```

pub fn add_one(x: i32) -> i32 {
    x + 1
}
