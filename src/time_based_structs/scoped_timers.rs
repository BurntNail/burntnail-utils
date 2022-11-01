use crate::memcache::MemoryCacher;
use std::{
    fmt::Display,
    sync::{Arc, Mutex},
    time::{Duration, Instant},
};

///Struct to time how long actions in a given scope last.
pub struct ScopedTimer {
    ///The message to print to the logs
    msg: String,
    ///When the action starts
    start_time: Instant,
}

impl ScopedTimer {
    ///Function to create a new `ScopedTimer` and start the timer
    pub fn new(msg: impl Display) -> Self {
        Self {
            msg: msg.to_string(),
            start_time: Instant::now(),
        }
    }
}

impl Drop for ScopedTimer {
    fn drop(&mut self) {
        #[cfg(feature = "tracing")]
        tracing::info!(time_taken=?self.start_time.elapsed(), msg=%self.msg);
        #[cfg(not(feature = "tracing"))]
        println!("{} took {:?}", self.msg, self.start_time.elapsed());
    }
}

///Same as [`ScopedTimer`], but updates a [`crate::memcache::MemoryCacher`] rather than adding to logs
pub struct ScopedToListTimer<'a, const N: usize>(&'a mut MemoryCacher<Duration, N>, Instant);

impl<'a, const N: usize> ScopedToListTimer<'a, N> {
    ///Creates a new `ScopedToListTimer`, and starts the timer
    pub fn new(t: &'a mut MemoryCacher<Duration, N>) -> Self {
        Self(t, Instant::now())
    }
}

impl<'a, const N: usize> Drop for ScopedToListTimer<'a, N> {
    fn drop(&mut self) {
        self.0.push(self.1.elapsed());
    }
}

///Thread-safe version of [`ScopedToListTimer`] that uses [`Arc`] and [`Mutex`] over `&mut`
pub struct ThreadSafeScopedToListTimer<const N: usize>(
    Arc<Mutex<MemoryCacher<Duration, N>>>,
    Instant,
);

impl<const N: usize> ThreadSafeScopedToListTimer<N> {
    ///Creates a new `ThreadSafeScopedToListTimer`, and starts the timer
    #[must_use]
    pub fn new(t: Arc<Mutex<MemoryCacher<Duration, N>>>) -> Self {
        Self(t, Instant::now())
    }
}

#[cfg(feature = "tracing")]
impl<const N: usize> Drop for ThreadSafeScopedToListTimer<N> {
    fn drop(&mut self) {
        use crate::error_ext::MutexExt;

        let elapsed = self.1.elapsed();
        let mut lock = self.0.lock_panic("locking memtimercache for timer");
        lock.push(elapsed);
    }
}

#[cfg(not(feature = "tracing"))]
impl<const N: usize> Drop for ThreadSafeScopedToListTimer<N> {
    fn drop(&mut self) {
        let elapsed = self.1.elapsed();
        let mut lock = self.0.lock().expect("locking memcache for timer");
        lock.push(elapsed);
    }
}
