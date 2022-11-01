//! I've found that I love Rust Type States, for which it is very repetitive and easiest to use a macro like the one provided
//!
//! For example, the following macro and code:
//!```rust
//! mod with_macro {
//!    mod sealed {
//!        /// Sealed so users of this library cannot make more type states.
//!        ///
//!        /// If you want different behaviour, use a trait like [`std::any::Any`] or [`std::drop::Drop`] in the macro invocation.
//!        pub trait SealedTrait {}
//!    }
//!    use burntnail_utils::generic_enum;
//!    use sealed::SealedTrait;
//!    generic_enum!(SealedTrait, (RocketMode -> "Trait for what state the rocket is in") => (InAssembly -> "Still being Assembled"), (OnLaunchPad -> "All assembled, and ready to be launched"), (Launched -> "In Space!!!"));
//!    /*various impls for a Rocket struct which has different methods for each stage*/
//! }
//!
//! ```
//! Produces the exact same as this
//!```rust
//! mod macro_expanded {
//!    mod sealed {
//!        /// Sealed so users of this library cannot make more type states.
//!        ///
//!        /// If you want different behaviour, use a trait like [`std::any::Any`] or [`std::drop::Drop`] in the macro invocation.
//!        pub trait SealedTrait {}
//!    }
//!
//!    ///Trait for what state the rocket is in
//!    pub trait RocketMode : sealed::SealedTrait {}
//!
//!    ///Still being Assembled
//!    pub struct InAssembly;
//!    impl sealed::SealedTrait for InAssembly {}
//!    impl RocketMode for InAssembly {}
//!
//!    ///All assembled, and ready to be launched
//!    pub struct OnLaunchPad;
//!    impl sealed::SealedTrait for OnLaunchPad {}
//!    impl RocketMode for OnLaunchPad {}
//!
//!    ///In Space!!!
//!    pub struct Launched;
//!    impl sealed::SealedTrait for Launched {}
//!    impl RocketMode for Launched {}
//!
//!    /* Rocket impls */
//! }
//! ```

///Provides any number of unit structs that implement a unit type
///
///Must pass in a `Sealed` trait for use in libraries, if you don't care use [`std::any::Any`] or [`std::ops::Drop`]
#[macro_export]
macro_rules! generic_enum {
    ($sealed_name:ident, ($trait_name:ident -> $trait_docs:literal) => $(($unit_struct_name:ident -> $docs:literal)),+) => {
        #[doc=$trait_docs]
        pub trait $trait_name : $sealed_name {}

        $(
            #[doc=$docs]
            #[derive(Copy, Clone, Debug)]
            pub struct $unit_struct_name;
            impl $sealed_name for $unit_struct_name {}
            impl $trait_name for $unit_struct_name {}
        )+
    };
}
