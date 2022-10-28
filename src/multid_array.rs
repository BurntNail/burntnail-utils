use crate::coords::Coords;
use std::ops::{Index, IndexMut};

///Struct for a 2D Array, backed by a [`Vec`]
pub struct TwoArray<T, const W: usize, const H: usize> {
    ///Base of the struct which holds all of the data
    backing: Vec<T>,
}

impl<T: Default, const W: usize, const H: usize> Default for TwoArray<T, W, H> {
    fn default() -> Self {
        Self {
            backing: Vec::with_capacity(W * H),
        }
    }
}
impl<T: Clone, const W: usize, const H: usize> TwoArray<T, W, H> {
    ///Instantiates a new `TwoArray`, with all elements being the default given
    ///
    ///If you want to just use the default T value, then consider using the [`Default`] trait implementation
    pub fn from_one_clone(default: T) -> Self {
        Self {
            backing: vec![default; W * H],
        }
    }
}

impl<T, const W: usize, const H: usize> Index<Coords<usize, 0, W, 0, H>> for TwoArray<T, W, H> {
    type Output = T;

    fn index(&self, index: Coords<usize, 0, W, 0, H>) -> &Self::Output {
        &self.backing[index.to_usize().unwrap_or_default()]
    }
}

impl<T, const W: usize, const H: usize> IndexMut<Coords<usize, 0, W, 0, H>> for TwoArray<T, W, H> {
    fn index_mut(&mut self, index: Coords<usize, 0, W, 0, H>) -> &mut Self::Output {
        &mut self.backing[index.to_usize().unwrap_or_default()]
    }
}
