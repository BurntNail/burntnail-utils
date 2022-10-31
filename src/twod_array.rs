use crate::{coords::Coords, error_ext::ToAnyhowNotErr};
use std::ops::{Index, IndexMut};

///Type alias for Usize coordinates used for Array indexing
pub type ArrayCoords<const W: usize, const H: usize> = Coords<usize, W, H>;

///Struct for a 2D Array, backed by a [`Vec`]
pub struct TwoArray<T, const W: usize, const H: usize> {
    ///Base of the struct which holds all of the data
    pub backing: Vec<T>,
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
impl<T, const W: usize, const H: usize> TwoArray<T, W, H> {
    ///Instantiates a new `TwoArray`, with all elements being from the function given
    pub fn from_function<F: Fn(ArrayCoords<W, H>) -> T>(f: F) -> Self {
        let mut backing = Vec::with_capacity(W * H);
        let mut index = ArrayCoords::default();

        backing.push(f(index));
        while index.increment() {
            backing.push(f(index));
        }

        Self { backing }
    }
}

impl<T, const W: usize, const H: usize> Index<ArrayCoords<W, H>> for TwoArray<T, W, H> {
    type Output = T;

    fn index(&self, index: ArrayCoords<W, H>) -> &Self::Output {
        &self.backing[index
            .to_usize()
            .unwrap_log_error_with_context(|| format!("getting index {index:?}"))]
    }
}
impl<T, const W: usize, const H: usize> IndexMut<ArrayCoords<W, H>> for TwoArray<T, W, H> {
    fn index_mut(&mut self, index: ArrayCoords<W, H>) -> &mut Self::Output {
        &mut self.backing[index
            .to_usize()
            .unwrap_log_error_with_context(|| format!("getting index {index:?}"))]
    }
}
impl<T, const W: usize, const H: usize> Index<(usize, usize)> for TwoArray<T, W, H> {
    type Output = T;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.backing[ArrayCoords::<W, H>::from(index)
            .to_usize()
            .unwrap_log_error_with_context(|| format!("getting index {index:?}"))]
    }
}
impl<T, const W: usize, const H: usize> IndexMut<(usize, usize)> for TwoArray<T, W, H> {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        &mut self.backing[ArrayCoords::<W, H>::from(index)
            .to_usize()
            .unwrap_log_error_with_context(|| format!("getting index {index:?}"))]
    }
}

///Iterator struct for [`TwoArray`]
pub struct TwoArrayIterator<T: Clone, const W: usize, const H: usize> {
    ///Base of the struct which holds all of the data
    pub backing: Vec<T>,
    ///The current position we're going over
    current_position: ArrayCoords<W, H>,
}

impl<T: Clone, const W: usize, const H: usize> Iterator for TwoArrayIterator<T, W, H> {
    type Item = (T, ArrayCoords<W, H>);

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_position.is_oob() {
            return None;
        }

        let item = (
            self.backing[self.current_position.to_usize().unwrap_or_default()].clone(),
            self.current_position,
        );
        self.current_position.increment();

        Some(item)
    }
}

impl<T: Clone, const W: usize, const H: usize> From<TwoArrayIterator<T, W, H>>
    for TwoArray<T, W, H>
{
    fn from(i: TwoArrayIterator<T, W, H>) -> Self {
        Self { backing: i.backing }
    }
}
impl<T: Clone, const W: usize, const H: usize> From<TwoArray<T, W, H>>
    for TwoArrayIterator<T, W, H>
{
    fn from(i: TwoArray<T, W, H>) -> Self {
        Self {
            backing: i.backing,
            current_position: ArrayCoords::InBounds(0, 0),
        }
    }
}

impl<T: Clone, const W: usize, const H: usize> IntoIterator for TwoArray<T, W, H> {
    type Item = (T, ArrayCoords<W, H>);
    type IntoIter = TwoArrayIterator<T, W, H>;

    fn into_iter(self) -> Self::IntoIter {
        self.into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn order_from_fn_test() {
        let array: TwoArray<ArrayCoords<3, 3>, 3, 3> = TwoArray::from_function(|c| c);

        let ac = ArrayCoords::InBounds;
        assert_eq!(
            array.backing,
            vec![
                ac(0, 0),
                ac(1, 0),
                ac(2, 0),
                ac(0, 1),
                ac(1, 1),
                ac(2, 1),
                ac(0, 2),
                ac(1, 2),
                ac(2, 2)
            ]
        );
    }

    #[test]
    fn index_iter_test() {
        let get_index = |cs| match cs {
            Coords::OutOfBounds => 0,
            Coords::InBounds(x, y) => x * 3 + y,
        };

        let array = TwoArray::from_function(get_index);
        let mut index = ArrayCoords::<3, 3>::default();

        for (el, ind) in array {
            assert_eq!(ind, index);
            assert_eq!(el, get_index(index));

            index.increment();
        }
        assert!(index.is_oob());
    }
}
