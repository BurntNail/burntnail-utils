use num_traits::Num;
use std::{fmt::Debug, ops::AddAssign};

///Utility type to hold a set of T coordinates (where T is a [`Num`] in an `(x, y)` format. Can also represent a piece which was taken.
///
/// (0, 0) is at the top left, with y counting the rows, and x counting the columns
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Coords<T: Num, const MAX_WIDTH: usize, const MAX_HEIGHT: usize> {
    ///The coordinate is currently off the board, or a taken piece
    OutOfBounds,
    ///The coordinate is currently on the board at these coordinates.
    InBounds(T, T),
}

impl<T: Num, const MAX_WIDTH: usize, const MAX_HEIGHT: usize> Default
    for Coords<T, MAX_WIDTH, MAX_HEIGHT>
{
    fn default() -> Self {
        Self::InBounds(T::zero(), T::zero())
    }
}

impl<T: Num + Debug, const MAX_WIDTH: usize, const MAX_HEIGHT: usize> Debug
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

impl<
        FROM: Num + PartialOrd<i32>,
        TO: Num + From<FROM> + PartialOrd<TO> + TryFrom<usize>,
        const MAX_WIDTH: usize,
        const MAX_HEIGHT: usize,
    > TryFrom<(FROM, FROM)> for Coords<TO, MAX_WIDTH, MAX_HEIGHT>
{
    type Error = anyhow::Error;

    fn try_from((x, y): (FROM, FROM)) -> Result<Self, Self::Error> {
        let (x, y) = (x.into(), y.into());

        if x < TO::zero() {
            bail!("x < 0")
        }
        if TO::try_from(MAX_WIDTH).map_or(false, |mw| x > mw) {
            bail!("x > {MAX_WIDTH}")
        }
        if y < TO::zero() {
            bail!("y < 0")
        }
        if TO::try_from(MAX_HEIGHT).map_or(false, |mh| y > mh) {
            bail!("y > {MAX_HEIGHT}")
        }

        Ok(Self::InBounds(x, y))
    }
}

impl<T: Num, const MAX_WIDTH: usize, const MAX_HEIGHT: usize> From<Coords<T, MAX_WIDTH, MAX_HEIGHT>>
    for Option<(T, T)>
{
    fn from(c: Coords<T, MAX_WIDTH, MAX_HEIGHT>) -> Self {
        match c {
            Coords::OutOfBounds => None,
            Coords::InBounds(x, y) => Some((x, y)),
        }
    }
}

impl<T: Num + TryFrom<usize> + Into<usize>, const MAX_WIDTH: usize, const MAX_HEIGHT: usize>
    Coords<T, MAX_WIDTH, MAX_HEIGHT>
{
    ///Provides an index with which to index a 1D array using the 2D coords, assuming there are 8 rows per column
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
        T: Num + AddAssign + TryFrom<usize> + Clone,
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
        let mut oob = false;

        if let Self::InBounds(cx, cy) = self {
            if T::try_from(MAX_WIDTH - 1).map_or(false, |mw| *cx == mw) {
                if T::try_from(MAX_HEIGHT - 1).map_or(false, |mh| *cy == mh) {
                    oob = true;
                } else {
                    *cx = T::zero();
                    *cy += T::one();
                }
            } else {
                *cx += T::one();
            }
        }
        if oob {
            *self = Self::OutOfBounds;
        }

        self.is_ib()
    }
}

impl<T: Num + Clone, const MAX_WIDTH: usize, const MAX_HEIGHT: usize>
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

impl<T: Num, const MAX_WIDTH: usize, const MAX_HEIGHT: usize> Coords<T, MAX_WIDTH, MAX_HEIGHT> {
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
