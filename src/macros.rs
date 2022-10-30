///Provides any number of unit structs that implement a unit type
///
///Must pass in a `Sealed` trait for use in libraries, if you don't care use [`std::any::Any`] or [`std::drop::Drop`]
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
