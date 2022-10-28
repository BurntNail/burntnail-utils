use num_traits::Num;
use std::fmt::Debug;

///Utility type to hold a set of T coordinates (where T is a [`Num`] in an `(x, y)` format. Can also represent a piece which was taken.
///
/// (0, 0) is at the top left, with y counting the rows, and x counting the columns
#[derive(Copy, Clone, PartialEq, Eq, Default)]
pub enum Coords<
    T: Num,
    const MIN_WIDTH: i32,
    const MAX_WIDTH: usize,
    const MIN_HEIGHT: i32,
    const MAX_HEIGHT: usize,
> {
    ///The coordinate is currently off the board, or a taken piece
    #[default]
    OutOfBounds,
    ///The coordinate is currently on the board at these coordinates.
    InBounds(T, T),
}

impl<
        T: Num + Debug,
        const MIN_WIDTH: i32,
        const MAX_WIDTH: usize,
        const MIN_HEIGHT: i32,
        const MAX_HEIGHT: usize,
    > Debug for Coords<T, MIN_WIDTH, MAX_WIDTH, MIN_HEIGHT, MAX_HEIGHT>
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
        F: Num + PartialOrd<i32>,
        T: Num + From<F>,
        const MIN_WIDTH: i32,
        const MAX_WIDTH: usize,
        const MIN_HEIGHT: i32,
        const MAX_HEIGHT: usize,
    > TryFrom<(F, F)> for Coords<T, MIN_WIDTH, MAX_WIDTH, MIN_HEIGHT, MAX_HEIGHT>
{
    type Error = anyhow::Error;

    fn try_from((x, y): (F, F)) -> Result<Self, Self::Error> {
        if x < MIN_WIDTH {
            bail!("x < {MIN_WIDTH}")
        }
        if x > i32::try_from(MAX_WIDTH).unwrap_or(i32::MAX) {
            bail!("x > {MAX_WIDTH}")
        }
        if y < MIN_HEIGHT {
            bail!("y < {MIN_HEIGHT}")
        }
        if y > i32::try_from(MAX_HEIGHT).unwrap_or(i32::MAX) {
            bail!("y > {MAX_HEIGHT}")
        }

        Ok(Self::InBounds(x.into(), y.into()))
    }
}

impl<
        T: Num + Clone,
        const MIN_WIDTH: i32,
        const MAX_WIDTH: usize,
        const MIN_HEIGHT: i32,
        const MAX_HEIGHT: usize,
    > Coords<T, MIN_WIDTH, MAX_WIDTH, MIN_HEIGHT, MAX_HEIGHT>
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

impl<
        T: Num,
        const MIN_WIDTH: i32,
        const MAX_WIDTH: usize,
        const MIN_HEIGHT: i32,
        const MAX_HEIGHT: usize,
    > From<Coords<T, MIN_WIDTH, MAX_WIDTH, MIN_HEIGHT, MAX_HEIGHT>> for Option<(T, T)>
{
    fn from(c: Coords<T, MIN_WIDTH, MAX_WIDTH, MIN_HEIGHT, MAX_HEIGHT>) -> Self {
        match c {
            Coords::OutOfBounds => None,
            Coords::InBounds(x, y) => Some((x, y)),
        }
    }
}

impl<
        T: Num + TryFrom<i32> + Into<usize>,
        const MIN_WIDTH: i32,
        const MAX_WIDTH: usize,
        const MIN_HEIGHT: i32,
        const MAX_HEIGHT: usize,
    > Coords<T, MIN_WIDTH, MAX_WIDTH, MIN_HEIGHT, MAX_HEIGHT>
{
    ///Provides an index with which to index a 1D array using the 2D coords, assuming there are 8 rows per column
    #[must_use]
    pub fn to_usize(self) -> Option<usize> {
        match self {
            Self::OutOfBounds => None,
            Self::InBounds(x, y) => {
                let multiplier =
                    T::try_from(i32::try_from(MAX_WIDTH).unwrap_or(i32::MAX) - MIN_WIDTH);
                match multiplier {
                    Ok(multiplier) => Some((y * multiplier + x).into()),
                    Err(_) => None,
                }
            }
        }
    }
}

impl<
        T: Num,
        const MIN_WIDTH: i32,
        const MAX_WIDTH: usize,
        const MIN_HEIGHT: i32,
        const MAX_HEIGHT: usize,
    > Coords<T, MIN_WIDTH, MAX_WIDTH, MIN_HEIGHT, MAX_HEIGHT>
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
