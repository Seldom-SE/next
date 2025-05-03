//! `next` is a crate that provides a trait that gets the next value. That value is the next in the
//! sequence implied by `PartialOrd`.

#![no_std]
#![warn(missing_docs)]

pub use next_macros::Next;

/// Allows getting the next sequential value
pub trait Next: Sized {
    /// The minimum value. It is the first in the sequence implied by `PartialOrd`.
    const MIN: Self;

    /// Gets the next value. That value is the next in the sequence implied by `PartialOrd`.
    fn next(self) -> Option<Self>;
}

impl Next for () {
    const MIN: Self = ();

    fn next(self) -> Option<Self> {
        None
    }
}

impl Next for bool {
    const MIN: Self = false;

    fn next(self) -> Option<Self> {
        (!self).then_some(true)
    }
}

macro_rules! next_int {
    ($ty:ty) => {
        impl Next for $ty {
            const MIN: Self = Self::MIN;

            fn next(self) -> Option<Self> {
                self.checked_add(1)
            }
        }
    };
}

next_int!(u8);
next_int!(u16);
next_int!(u32);
next_int!(u64);
next_int!(u128);
next_int!(usize);
next_int!(i8);
next_int!(i16);
next_int!(i32);
next_int!(i64);
next_int!(i128);
next_int!(isize);

macro_rules! next_float {
    ($ty:ty) => {
        impl Next for $ty {
            const MIN: Self = Self::NEG_INFINITY;

            fn next(self) -> Option<Self> {
                (self != Self::INFINITY).then(|| self.next_up())
            }
        }
    };
}

next_float!(f32);
next_float!(f64);
