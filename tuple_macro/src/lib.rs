#![forbid(unsafe_code)]
extern crate proc_macro;
use proc_macro::TokenStream;

fn generic_type_param(n: usize) -> String {
    (1..=n).map(|i| format!("T{}, ", i)).collect::<String>()
}

fn from_tup_fn_ident(n: usize) -> String {
    (1..=n)
        .map(|i| format!("t{}.into(), ", i))
        .collect::<String>()
}

fn from_tup_type_bound(n: usize) -> String {
    (1..=n)
        .map(|i| format!("From<T{}> + ", i))
        .collect::<String>()
}

fn from_tup_value_ident(n: usize) -> String {
    (1..=n).map(|i| format!("t{}, ", i)).collect::<String>()
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

#[rustfmt::skip]
fn from_tup_trait_code(n: usize) -> String {
    (1..=n).map(|i| format!(
"   #[doc = \"Converts tuple ({type_doc}) to array [Self; {i}].\"]
    fn from_{i}<{type_param}>(tup: ( {type_param} ) ) -> [Self; {i}]
        where
             Self: {type_bound};

",
        type_param = generic_type_param(i),
        type_bound = from_tup_type_bound(i),
        type_doc = generic_type_param(i).trim_end(),
        i = i,)).collect::<String>()
}

#[rustfmt::skip]
fn from_tup_impl_code(n: usize) -> String {
    (1..=n).map(|i| format!(
"   #[doc = \"Converts tuple ({type_doc}) to array [Self; {i}].\"]
    #[inline] 
    fn from_{i}<{type_param}>(tup: ( {type_param} ) ) -> [Self; {i}]
        where
             Self: {type_bound},
    {{
        let ( {value_ident} ) = tup;
        [ {fn_ident}]
    }}
",
        type_param = generic_type_param(i),
        fn_ident = from_tup_fn_ident(i),
        type_bound = from_tup_type_bound(i),
        value_ident = from_tup_value_ident(i),
        type_doc = generic_type_param(i).trim_end(),
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

macro_rules! from_tup_trait {
    ($to:expr) => {
        #[proc_macro]
        pub fn tup_from_trait(_item: TokenStream) -> TokenStream {
            from_tup_trait_code($to).parse().unwrap()
        }

        #[proc_macro]
        pub fn tup_from_impl(_item: TokenStream) -> TokenStream {
            from_tup_impl_code($to).parse().unwrap()
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


#[cfg(all(not(feature = "from_tup_16"), not(feature = "from_tup_32"), not(feature = "from_tup_64")))]
from_tup_trait!(8);

#[cfg(all(feature = "from_tup_16", not(feature = "from_tup_32"), not(feature = "from_tup_64")))]
from_tup_trait!(16);

#[cfg(all(feature = "from_tup_16", feature = "from_tup_32", not(feature = "from_tup_64")))]
from_tup_trait!(32);

#[cfg(all(feature = "from_tup_32", not(feature = "from_tup_16"), not(feature = "from_tup_64")))]
from_tup_trait!(32);

#[cfg(all(feature = "from_tup_16", feature = "from_tup_64", not(feature = "from_tup_32")))]
from_tup_trait!(64);

#[cfg(all(feature = "from_tup_32", feature = "from_tup_64", not(feature = "from_tup_16")))]
from_tup_trait!(64);

#[cfg(all(feature = "from_tup_64", not(feature = "from_tup_16"), not(feature = "from_tup_32")))]
from_tup_trait!(64);

#[cfg(all(feature = "from_tup_16", feature = "from_tup_32", feature = "from_tup_64"))]
from_tup_trait!(64);
