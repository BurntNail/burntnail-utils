//! An Either type for representing something that can be A or B. Similar ergonomics to a [`std::result::Result`], and if need be you can convert easily to one.
//!
//! Includes lots of transformers to get values out of an option, as well as conditional implementations like [`std::clone::Clone`] and [`std::fmt::Debug`]

use std::fmt::{Debug, Formatter};

///Enum which can represent one of two values
///
///The Same as an `(Option<A>, Option<B>)` where one [`Option`] must always be [`Option::Some`] and the other must be [`Option::None`]
pub enum Either<L, R> {
    ///The First variant of [`Either`]
    Left(L),
    ///The second variant of [`Either`]
    Right(R),
}

impl<L, R> Either<L, R> {
    ///Constructor for [`Either::Left`] which uses [`Into::into`]
    pub fn l(a: impl Into<L>) -> Self {
        Self::Left(a.into())
    }

    ///Constructor for [`Either::Right`] which uses [`Into::into`]
    pub fn r(b: impl Into<R>) -> Self {
        Self::Right(b.into())
    }

    ///Utility function for checking which side it is. Here we check if it is the [`Either::Left`] variant
    pub const fn is_left(&self) -> bool {
        matches!(self, Self::Left(_))
    }

    ///Utility function for checking which side it is. Here we check if it is the [`Either::Right`] variant
    pub const fn is_right(&self) -> bool {
        matches!(self, Self::Right(_))
    }

    //region transformers
    ///Utility function for [`Either::Left`] Vs [`Either::Right`] if you need nice variable names
    pub const fn to_unit(&self) -> Either<(), ()> {
        if self.is_left() {
            Either::Left(())
        } else {
            Either::Right(())
        }
    }

    ///Function to check if this is [`Either::Left`], and if so return [`Some`] of that, else [`None`]
    ///
    ///Returns an Owned `L`
    ///
    ///Non-const due to [E0493](https://doc.rust-lang.org/error-index.html#E0493)
    #[allow(clippy::missing_const_for_fn)]
    pub fn to_left(self) -> Option<L> {
        if let Self::Left(l) = self {
            Some(l)
        } else {
            None
        }
    }
    ///Function to check if this is [`Either::Left`], and if so return [`Some`] of that, else [`None`]
    ///
    ///Returns a reference to an `L`
    pub const fn ref_left(&self) -> Option<&L> {
        if let Self::Left(l) = self {
            Some(l)
        } else {
            None
        }
    }
    ///Function to check if this is [`Either::Left`], and if so return [`Some`] of that, else [`None`]
    ///
    ///Returns a mutable reference to an `L`
    pub fn mut_ref_left(&mut self) -> Option<&mut L> {
        if let Self::Left(l) = self {
            Some(l)
        } else {
            None
        }
    }

    ///Function to check if this is [`Either::Right`], and if so return [`Some`] of that, else [`None`]
    ///
    ///Returns an Owned `L`
    ///
    ///Non-const due to [E0493](https://doc.rust-lang.org/error-index.html#E0493)
    #[allow(clippy::missing_const_for_fn)]
    pub fn to_right(self) -> Option<R> {
        if let Self::Right(r) = self {
            Some(r)
        } else {
            None
        }
    }
    ///Function to check if this is [`Either::Right`], and if so return [`Some`] of that, else [`None`]
    ///
    ///Returns a reference to an `L`
    pub const fn ref_right(&self) -> Option<&R> {
        if let Self::Right(r) = self {
            Some(r)
        } else {
            None
        }
    }
    ///Function to check if this is [`Either::Right`], and if so return [`Some`] of that, else [`None`]
    ///
    ///Returns a mutable reference to an `L`
    pub fn mut_ref_right(&mut self) -> Option<&mut R> {
        if let Self::Right(r) = self {
            Some(r)
        } else {
            None
        }
    }

    ///Converts Either<L, R> to Result<L, R>
    #[allow(clippy::missing_errors_doc, clippy::missing_const_for_fn)] //no need, issue with destructors
    pub fn to_result(self) -> Result<L, R> {
        match self {
            Self::Left(l) => Ok(l),
            Self::Right(r) => Err(r),
        }
    }

    //endregion
    //TODO: Work out more elegant way (maybe macros) to do above and below transformers
}

impl<L: Clone, R: Clone> Either<L, R> {
    ///Function to check if this is [`Either::Left`], and if so return [`Some`] of that, else [`None`]
    pub fn clone_left(&self) -> Option<L> {
        if let Self::Left(l) = self.clone() {
            Some(l)
        } else {
            None
        }
    }

    ///Function to check if this is [`Either::Right`], and if so return [`Some`] of that, else [`None`]
    pub fn clone_right(&self) -> Option<R> {
        if let Self::Right(r) = self.clone() {
            Some(r)
        } else {
            None
        }
    }
}

impl<T> Either<T, T> {
    ///If `L` == `R` then this function will return an `L` - useful for when the [`Either`] side signifies something, but always returns the same type.
    #[allow(clippy::missing_const_for_fn)] //Cannot be const as destructors cannot be const - Github error 8874
    pub fn one_type(self) -> T {
        match self {
            Self::Left(l) => l,
            Self::Right(r) => r,
        }
    }
}

impl<L: Debug, R: Debug> Debug for Either<L, R> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Left(arg0) => f.debug_tuple("Left").field(arg0).finish(),
            Self::Right(arg0) => f.debug_tuple("Right").field(arg0).finish(),
        }
    }
}

impl<L: Clone, R: Clone> Clone for Either<L, R> {
    fn clone(&self) -> Self {
        match self {
            Self::Left(l) => Self::Left(l.clone()),
            Self::Right(r) => Self::Right(r.clone()),
        }
    }
}
