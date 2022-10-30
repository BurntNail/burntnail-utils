use crate::time_based_structs::do_on_interval::{DoOnInterval, UpdateOnCheck};
use std::{
    fmt::Debug,
    ops::{AddAssign, Div},
    vec::IntoIter,
};

///Struct to hold a list of items that only get updated on a [`DoOnInterval`], with a circular cache that overwrites the oldest items if there isn't any free space.
///
///Has 2 generic properties - `T` for the type stored, and `N` for the size of the backing array
#[derive(Debug)]
pub struct MemoryCacher<T, const N: usize> {
    ///Holds all the data
    data: Vec<T>,
    ///Marks whether or not the array is full of data - useful for after it wraps around
    full: bool,
    ///Holds the index of the last data written in.
    ///
    ///Unless the list is full, this index should not contain data
    index: usize,

    ///Holds a timer in case we only want to write data on intervals rather than whenever `add` is called
    timer: Option<DoOnInterval<UpdateOnCheck>>,
}

impl<T: Copy, const N: usize> Default for MemoryCacher<T, N> {
    fn default() -> Self {
        Self {
            data: Vec::with_capacity(N),
            full: false,
            index: 0,
            timer: None,
        }
    }
}

impl<T: Copy, const N: usize> MemoryCacher<T, N> {
    ///Creates a blank Memory Cacher
    #[must_use]
    pub fn new(t: Option<DoOnInterval<UpdateOnCheck>>) -> Self {
        Self {
            timer: t,
            ..Default::default()
        }
    }

    ///Adds an element to the list on the following conditions:
    /// - there are no elements
    /// - there is a [`DoOnInterval`] timer, and we can use it
    pub fn push(&mut self, t: T) {
        let can = self.timer.as_mut().map_or(true, DoOnInterval::can_do);

        if can {
            if self.full {
                self.data[self.index] = t;
            } else {
                self.data.push(t);
            }

            self.index = (self.index + 1) % N;

            if let Some(t) = &mut self.timer {
                t.update_timer();
            }
        }
    }

    ///Returns whether or not the list is empty
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    ///Gets all of the elements, with order unimportant
    #[must_use]
    #[allow(clippy::missing_const_for_fn)] //destructor issues
    pub fn get_all(self) -> Vec<T> {
        self.data
    }

    ///Gets all of the elements, copying all elements to avoid ownership issues
    #[must_use]
    pub fn get_all_copy(&self) -> Vec<T> {
        self.data.clone()
    }
}

impl<T: Copy, const N: usize> IntoIterator for MemoryCacher<T, N> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.get_all().into_iter()
    }
}

///Creates an average function for an {integer} type
macro_rules! average_impl {
    ($($t:ty => $name:ident),+) => {
        $(
            impl<T, const N: usize> MemoryCacher<T, N>
            where
                T: Div<$t> + AddAssign + Default + Clone + Copy + Debug,
                T::Output: Default,
            {
                ///Function to get the average of the items in the list
                #[must_use]
                pub fn $name(&self) -> T::Output {
                    if self.is_empty() {
                        return T::Output::default();
                    }

                    let mut total = T::default();
                    let mut count = 0;

                    for el in self.get_all_copy().into_iter() {
                        total += el;
                        count += 1;
                    }

                    total / count
                }
            }
        )+
    };
}
///Creates an average function for a {float} type
macro_rules! average_fp_impl {
    ($($t:ty => $name:ident),+) => {
        $(
            impl<T, const N: usize> MemoryCacher<T, N>
            where
                T: Div<$t> + AddAssign + Default + Clone + Copy + Debug + Default,
                T::Output: Default
            {
                ///Function to get the average of the items in the list
                #[must_use]
                pub fn $name(&self) -> T::Output {
                    if self.is_empty() {
                        return T::Output::default();
                    }

                    let mut total = T::default();
                    let mut count = 0.0;

                    for el in self.get_all_copy().into_iter() {
                        total += el;
                        count += 1.0;
                    }

                    total / count
                }
            }
        )+
    };
}

average_impl!(u8 => average_u8, u16 => average_u16, u32 => average_u32, u64 => average_u64, u128 => average_u128, i8 => average_i8, i16 => average_i16, i32 => average_i32, i64 => average_i64, i128 => average_i128);
average_fp_impl!(f32 => average_f32, f64 => average_f64);

#[cfg(test)]
mod tests {
    use crate::memcache::MemoryCacher;

    #[test]
    pub fn hand_constructed_get_all() {
        let vec = vec![100_i32; 10];
        let list: MemoryCacher<_, 10> = MemoryCacher {
            data: vec![100_i32; 10],
            full: true,
            index: 9,
            timer: None,
        };

        assert_eq!(vec, list.get_all());
    }

    #[test]
    pub fn no_timer_basic_push() {
        let mut full_list = MemoryCacher::<_, 10>::new(None);
        let mut half_full_list = MemoryCacher::<_, 20>::new(None);

        let base_10 = (0..10).into_iter().collect::<Vec<i32>>();
        for i in base_10.clone() {
            full_list.push(i);
            half_full_list.push(i);
        }
        assert_eq!(full_list.get_all_copy(), base_10.clone());
        assert_eq!(half_full_list.get_all_copy(), base_10.clone());
    }
}
