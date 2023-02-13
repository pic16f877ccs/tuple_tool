#![forbid(unsafe_code)]
extern crate proc_macro;
use proc_macro::TokenStream;

fn generic_type_param(n: usize) -> String {
    (1..=n).map(|i| format!("T{}, ", i)).collect::<String>()
}

#[rustfmt::skip]
fn tup_len_impls_code(n: usize) -> String {
    (1..=n).map(|i| format!(
"impl<{type_param}> TupLen for ({type_param})
    {{ #[inline] fn len(&self) -> usize
{{ {i} }} }}\n",
        type_param = generic_type_param(i),
        i = i,)).collect::<String>()
}

macro_rules! tuple_len_impl {
    ($to:expr) => {
        #[proc_macro]
        pub fn tuple_length(_item: TokenStream) -> TokenStream {
            tup_len_impls_code($to).parse().unwrap()
        }
    };
}

#[cfg(all(not(feature = "tup_len_16"), not(feature = "tup_len_32"), not(feature = "tup_len_64")))]
tuple_len_impl!(8);

#[cfg(all(feature = "tup_len_16", not(feature = "tup_len_32"), not(feature = "tup_len_64")))]
tuple_len_impl!(16);

#[cfg(all(feature = "tup_len_16", feature = "tup_len_32", not(feature = "tup_len_64")))]
tuple_len_impl!(32);

#[cfg(all(feature = "tup_len_32", not(feature = "tup_len_16"), not(feature = "tup_len_64")))]
tuple_len_impl!(32);

#[cfg(all(feature = "tup_len_16", feature = "tup_len_64", not(feature = "tup_len_32")))]
tuple_len_impl!(64);

#[cfg(all(feature = "tup_len_32", feature = "tup_len_64", not(feature = "tup_len_16")))]
tuple_len_impl!(64);

#[cfg(all(feature = "tup_len_64", not(feature = "tup_len_16"), not(feature = "tup_len_32")))]
tuple_len_impl!(64);

#[cfg(all(feature = "tup_len_16", feature = "tup_len_32", feature = "tup_len_64"))]
tuple_len_impl!(64);
