#![forbid(unsafe_code)]
extern crate proc_macro;
use proc_macro::TokenStream;

fn create_type_param(n: usize) -> String {
    (1..=n).map(|i| format!("T{}, ", i)).collect::<String>()
}

#[rustfmt::skip]
fn create_impls_code(n: usize) -> String {
    (1..=n).map(|i| format!(
"impl<{type_param}> TupLen for ({type_param})
    {{ #[inline] fn len(&self) -> usize
{{ {i} }} }}\n",
        type_param = create_type_param(i),
        i = i,)).collect::<String>()
}

macro_rules! macro_impl {
    ($to:expr) => {
        #[proc_macro]
        pub fn tuple_length(_item: TokenStream) -> TokenStream {
            create_impls_code($to).parse().unwrap()
        }
    };
}

#[cfg(all(not(feature = "16"), not(feature = "32"), not(feature = "64")))]
macro_impl!(8);

#[cfg(all(feature = "16", not(feature = "32"), not(feature = "64")))]
macro_impl!(16);

#[cfg(all(feature = "16", feature = "32", not(feature = "64")))]
macro_impl!(32);

#[cfg(all(feature = "32", not(feature = "16"), not(feature = "64")))]
macro_impl!(32);

#[cfg(all(feature = "16", feature = "64", not(feature = "32")))]
macro_impl!(64);

#[cfg(all(feature = "32", feature = "64", not(feature = "16")))]
macro_impl!(64);

#[cfg(all(feature = "64", not(feature = "16"), not(feature = "32")))]
macro_impl!(64);

#[cfg(all(feature = "16", feature = "32", feature = "64"))]
macro_impl!(64);
