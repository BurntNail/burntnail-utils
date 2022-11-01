//!Contains a type for containing a 2D coordinates, with [`usize`] bounds
//!
//! ## General Use
//! Coordinates generally have bounds, and here, you can use that in combination with all of the other trait implementations to ensure your coordinate is always valid.
//!
//! For example, when making a coordinate, if the provided coordinates are out of bounds, then the enum variant will be Out Of Bounds. This can also occur if you add coordinates and the result is OOB.
//!```rust
//! use burntnail_utils::coords::Coords;
//!
//! let coords: Coords<i32, 100, 100> = Coords::from((1000, 1000));
//! assert!(coords.is_oob());
//!
//!
//! let a: Coords<i32, 100, 100> = Coords::from((75, 75));
//! assert!(a.is_ib());
//! assert!((a + a).is_oob());
//!
//!
//! let mut b: Coords<i32, 100, 100> = Coords::from((98, 99));
//! assert!(b.is_ib()); //98, 99 is inbounds
//!
//! assert!(b.increment()); //if we increment and stay inbounds then increment returns true
//! assert!(b.is_ib()); //99, 99 is inbounds
//!
//! assert!(!b.increment()); //now we go oob, so increment returns false
//! assert!(b.is_oob()); //0, 100 is oob
//! ```
//!
//! There are also lots of conditional trait implementations, as you can see. For example, if your `T` provides [`std::fmt::Debug`], then the Coordinates will also be debuggable.
//!
//! ## Array-Related Uses
//!
//! These Coordinates can also be used in conjunction with arrays.
//!
//! For example, they can be used to index into [`crate::twod_array::TwoArray`] assuming `Coords::MAX_WIDTH == TwoArray::WIDTH && Coords::MAX_HEIGHT == TwoArray::HEIGHT`.
//!
//! Also, if you're running a 1D backing for a homemade 2D array, if `T: Into<usize>`, then you can get a usize index to index an array with.

use num_traits::Num;
use std::{
    fmt::Debug,
    ops::{Add, AddAssign, Div, Mul, Sub},
};
///Utility type to hold a set of T coordinates (where T is a [`Num`] in an `(x, y)` format. Can also represent a piece which was taken. If you want coordinates for anywhere, just use `usize::MAX` for the bounds
///
/// (0, 0) is at the top left, with y counting the rows, and x counting the columns.
///
/// NB: These bounds are **exclusive**
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Coords<T: Num + TryFrom<usize>, const MAX_WIDTH: usize, const MAX_HEIGHT: usize> {
    ///The coordinate is currently off the board, or a taken piece
    ///
    ///Any operation performed on or with Out of Bounds coordinates will return Out of Bounds coordinates.
    OutOfBounds,
    ///The coordinate is currently on the board at these coordinates.
    InBounds(T, T),
}

///Utility type for coordinates that can exist without maximum x or y positions.
pub type UnboundedCoord<T> = Coords<T, { usize::MAX }, { usize::MAX }>;

impl<T: Num + TryFrom<usize>, const MAX_WIDTH: usize, const MAX_HEIGHT: usize> Default
    for Coords<T, MAX_WIDTH, MAX_HEIGHT>
{
    fn default() -> Self {
        Self::InBounds(T::zero(), T::zero())
    }
}

impl<T: Num + Debug + TryFrom<usize>, const MAX_WIDTH: usize, const MAX_HEIGHT: usize> Debug
    for Coords<T, MAX_WIDTH, MAX_HEIGHT>
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::OutOfBounds => f.debug_struct("Coords").finish(),
            Self::InBounds(x, y) => f
                .debug_struct("Coords")
                .field("x", x)
                .field("y", y)
                .finish(),
        }
    }
}

impl<T: Num + TryFrom<usize> + PartialOrd, const MAX_WIDTH: usize, const MAX_HEIGHT: usize>
    From<(T, T)> for Coords<T, MAX_WIDTH, MAX_HEIGHT>
{
    fn from((x, y): (T, T)) -> Self {
        if T::try_from(MAX_WIDTH).map_or(false, |mw| x >= mw)
            || T::try_from(MAX_HEIGHT).map_or(false, |mh| y >= mh)
        {
            Self::OutOfBounds
        } else {
            Self::InBounds(x, y)
        }
    }
}

impl<T: Num + TryFrom<usize> + Into<usize>, const MAX_WIDTH: usize, const MAX_HEIGHT: usize>
    Coords<T, MAX_WIDTH, MAX_HEIGHT>
{
    ///Provides an index with which to index a 1D array using the 2D coords, assuming a starting position of (0, 0)
    #[must_use]
    pub fn to_usize(self) -> Option<usize> {
        match self {
            Self::OutOfBounds => None,
            Self::InBounds(x, y) => match T::try_from(MAX_WIDTH) {
                Ok(multiplier) => Some((y * multiplier + x).into()),
                Err(_) => None,
            },
        }
    }
}

impl<
        T: Num + AddAssign + TryFrom<usize> + TryInto<usize> + PartialOrd,
        const MAX_WIDTH: usize,
        const MAX_HEIGHT: usize,
    > Coords<T, MAX_WIDTH, MAX_HEIGHT>
{
    ///Utility function to incremenent the coordinate.
    ///
    ///Goes x then y, and if reaches bottom right, then goes OOB.
    ///
    ///Returns true if result isn't OOB
    pub fn increment(&mut self) -> bool {
        let mut oob = self.is_oob();

        if let Self::InBounds(cx, cy) = self {
            if T::try_from(MAX_WIDTH - 1).map_or(false, |mw| *cx >= mw) {
                if T::try_from(MAX_HEIGHT - 1).map_or(false, |mh| *cy >= mh) {
                    oob = true;
                } else {
                    *cx = T::zero();
                    *cy += T::one();
                }
            } else {
                *cx += T::one();
            }
        }
        if !self.is_oob() && oob {
            *self = Self::OutOfBounds;
        }

        !oob
    }
}

impl<T: Num + Clone + TryFrom<usize>, const MAX_WIDTH: usize, const MAX_HEIGHT: usize>
    Coords<T, MAX_WIDTH, MAX_HEIGHT>
{
    ///Provides a utility function for turning `Coords` to an `Option<(T, T)>`
    ///
    ///NB: Clones T
    #[must_use]
    pub fn to_option(&self) -> Option<(T, T)> {
        match self.clone() {
            Self::OutOfBounds => None,
            Self::InBounds(x, y) => Some((x, y)),
        }
    }

    ///Provides the X part of the coordinate
    #[must_use]
    pub fn x(&self) -> Option<T> {
        self.to_option().map(|(x, _)| x)
    }
    ///Provides the Y part of the coordinate
    #[must_use]
    pub fn y(&self) -> Option<T> {
        self.to_option().map(|(_, y)| y)
    }
}

impl<T: Num + TryFrom<usize>, const MAX_WIDTH: usize, const MAX_HEIGHT: usize>
    Coords<T, MAX_WIDTH, MAX_HEIGHT>
{
    ///Utility function for whether or not it is out of bounds
    #[must_use]
    pub const fn is_oob(&self) -> bool {
        matches!(self, Self::OutOfBounds)
    }

    ///Utility function for whether or not it is out of bounds
    #[must_use]
    pub const fn is_ib(&self) -> bool {
        matches!(self, Self::InBounds(_, _))
    }
}

impl<T: Num + TryFrom<usize> + PartialOrd, const MAX_WIDTH: usize, const MAX_HEIGHT: usize> Add
    for Coords<T, MAX_WIDTH, MAX_HEIGHT>
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Self::OutOfBounds, _) | (_, Self::OutOfBounds) => Self::OutOfBounds,
            (Self::InBounds(ax, ay), Self::InBounds(bx, by)) => {
                let x: T = ax + bx;
                let y: T = ay + by;
                Self::from((x, y))
            }
        }
    }
}
impl<T: Num + TryFrom<usize> + PartialOrd, const MAX_WIDTH: usize, const MAX_HEIGHT: usize> Sub
    for Coords<T, MAX_WIDTH, MAX_HEIGHT>
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Self::OutOfBounds, _) | (_, Self::OutOfBounds) => Self::OutOfBounds,
            (Self::InBounds(ax, ay), Self::InBounds(bx, by)) => {
                let x: T = ax - bx;
                let y: T = ay - by;
                Self::from((x, y))
            }
        }
    }
}
impl<
        T: Num + TryFrom<usize> + PartialOrd + Mul + Copy,
        const MAX_WIDTH: usize,
        const MAX_HEIGHT: usize,
    > Mul<T> for Coords<T, MAX_WIDTH, MAX_HEIGHT>
{
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        match self {
            Self::OutOfBounds => Self::OutOfBounds,
            Self::InBounds(x, y) => Self::from((x * rhs, y * rhs)),
        }
    }
}
impl<
        T: Num + TryFrom<usize> + PartialOrd + Div + Copy,
        const MAX_WIDTH: usize,
        const MAX_HEIGHT: usize,
    > Div<T> for Coords<T, MAX_WIDTH, MAX_HEIGHT>
{
    type Output = Self;

    fn div(self, rhs: T) -> Self::Output {
        match self {
            Self::OutOfBounds => Self::OutOfBounds,
            Self::InBounds(x, y) => Self::from((x / rhs, y / rhs)),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::coords::Coords;

    #[test]
    fn increment_test() {
        let mut coord = Coords::<_, 3, 3>::default();

        assert_eq!(coord, Coords::InBounds(0, 0));
        assert!(coord.increment());
        assert_eq!(coord, Coords::InBounds(1, 0));
        assert!(coord.increment());
        assert_eq!(coord, Coords::InBounds(2, 0));
        assert!(coord.increment());

        assert_eq!(coord, Coords::InBounds(0, 1));
        assert!(coord.increment());
        assert_eq!(coord, Coords::InBounds(1, 1));
        assert!(coord.increment());
        assert_eq!(coord, Coords::InBounds(2, 1));
        assert!(coord.increment());

        assert_eq!(coord, Coords::InBounds(0, 2));
        assert!(coord.increment());
        assert_eq!(coord, Coords::InBounds(1, 2));
        assert!(coord.increment());
        assert_eq!(coord, Coords::InBounds(2, 2));

        assert!(!coord.increment());
        assert!(coord.is_oob());
    }
}
