//! Utility macros

macro_rules! impl_all {
    ($impl_macro:ident: $($id:ident),*) => {
        $(
            $impl_macro!($id);
        )*
    }
}
