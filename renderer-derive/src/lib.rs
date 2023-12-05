mod get_variant;
mod is_variant;

use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

use self::get_variant::impl_get_variant;
use self::is_variant::impl_is_variant;

#[proc_macro_derive(IsVariant)]
pub fn derive_is_variant(input: TokenStream) -> TokenStream {
    // See https://doc.servo.org/syn/derive/struct.DeriveInput.html
    let ast: DeriveInput = parse_macro_input!(input as DeriveInput);
    impl_is_variant(ast)
}

#[proc_macro_derive(GetVariant)]
pub fn derive_get_variant(input:TokenStream)->TokenStream{
    let ast:DeriveInput = parse_macro_input!(input as DeriveInput);
    impl_get_variant(ast)
}
