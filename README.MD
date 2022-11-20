# BurntNail Utils

Hey! You've found my general utilities for use with rust.
There are quite a few now, and it includes optional `tracing` support, with standard `println` and `eprintln` as fallbacks.

## Error Extensions
Mainly designed as a utility for use with `anyhow` and `color-eyre`, so includes ways to turn things like `Option` and `PosionErr` into Anyhow Errors.

Also includes a trait for quickly logging errors, as an alternative to `unwrap` and `expect`

## Time Based Structs
### Scoped Timer
Includes a `ScopedTimer`, where you make a timer, and then on `Drop`, it prints out the time it took and a message.

There is also a `ThreadSafeScopedToListTimer`, which sends the result to a `MemoryCacher` which holds `Duration` structs.

## Do On Interval
A useful struct to hold a timer, and whenever you want to do an action on an interval, you check with it, and it'll say. After, then you either update it manually, or use a guard.

## Memory Cacher
Basically, a circular queue of fixed size. Used to be unsafe using `MaybeUninit`, but I then found that it was around 10x faster when I backed it with a normal `Vec`.

## Coordinates
A struct for coordinates, with generic interior types and maximum widths/heights.

## Either
A basic Either enum type with Left, Right and some conversion methods.

## Piston Cacher
A struct for caching piston2d images based on file paths.

## Multidimensional Arrays
Currently only contains a 2D array, which can be indexed using either the native coordinates or `(usize, usize)`



## TODO
 - Change `ThreadSafeScopedToListTimer` to use different things which can be pushed to (maybe new trait?)
 - Rename some things to make it more clear
 - Docu-Tests and more examples