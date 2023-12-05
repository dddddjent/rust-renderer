use proc_macro::TokenStream;
use proc_macro2::{Span, TokenStream as TokenStream2};

use quote::{format_ident, quote, quote_spanned};
use syn::spanned::Spanned;
use syn::{Data, DeriveInput, Error, Fields};

macro_rules! derive_error {
    ($string: tt) => {
        Error::new(Span::call_site(), $string)
            .to_compile_error()
            .into()
    };
}

// https://crates.io/crates/convert_case
use convert_case::{Case, Casing};

pub fn impl_is_variant(ast: DeriveInput) -> TokenStream {
    // get enum name
    let enum_ident = &ast.ident;
    let data = &ast.data;

    let mut variant_checker_functions = TokenStream2::new();

    match data {
        Data::Enum(enum_data) => {
            for variant in &enum_data.variants {
                let variant_ident = &variant.ident;
                let fields_in_variant = match &variant.fields {
                    Fields::Unnamed(_) => quote_spanned! {variant.span()=> (..) },
                    Fields::Unit => quote_spanned! { variant.span()=> },
                    Fields::Named(_) => quote_spanned! {variant.span()=> {..} },
                };

                let is_variant_func_ident =
                    format_ident!("is_{}", variant_ident.to_string().to_case(Case::Snake));

                // Here we construct the function for the current variant
                variant_checker_functions.extend(quote_spanned! {variant.span()=>
                    fn #is_variant_func_ident(&self) -> bool {
                        match self {
                            #enum_ident::#variant_ident #fields_in_variant => true,
                            _ => false,
                        }
                    }
                });
            }
        }
        _ => return derive_error!("IsVariant is only implemented for enums"),
    };

    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();

    let expanded = quote! {
        impl #impl_generics #enum_ident #ty_generics #where_clause {
            // variant_checker_functions gets replaced by all the functions
            // that were constructed above
            #variant_checker_functions
        }
    };

    TokenStream::from(expanded)
}
