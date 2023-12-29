use convert_case::{Case, Casing};
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote};
use syn::DeriveInput;

macro_rules! derive_error {
    ($string: tt) => {
        syn::Error::new(proc_macro2::Span::call_site(), $string)
            .to_compile_error()
            .into()
    };
}

pub fn impl_get_variant(ast: DeriveInput) -> TokenStream {
    let enum_ident = ast.ident;
    let enum_name = enum_ident.to_string();
    let data = ast.data;

    let mut get_variant_functions = TokenStream2::new();

    match data {
        syn::Data::Enum(enum_data) => {
            for variant in &enum_data.variants {
                let variant_ident = &variant.ident;
                let variant_name = variant_ident.to_string();
                let function_name =
                    format_ident!("get_{}", variant_ident.to_string().to_case(Case::Snake));
                let function_mut_name =
                    format_ident!("get_{}_mut", variant_ident.to_string().to_case(Case::Snake));

                if let syn::Fields::Named(fields_named) = &variant.fields {
                    let fields = &fields_named.named;
                    if fields.len() > 1 {
                        continue;
                    }
                    let field_ident = fields[0].ident.as_ref().unwrap();
                    let field_ty = &fields[0].ty;
                    get_variant_functions.extend(quote!(
                        #[inline]
                        pub fn #function_name(&self) -> Result<&#field_ty,String>{
                            match self {
                                #enum_ident::#variant_ident{#field_ident}=>Ok(#field_ident),
                                _=>Err(format!("This enum variant is not {}::{}!",#enum_name,#variant_name)),
                            }
                        }
                        #[inline]
                        pub fn #function_mut_name(&mut self) -> Result<&mut #field_ty,String>{
                            match self {
                                #enum_ident::#variant_ident{#field_ident}=>Ok(#field_ident),
                                _=>Err(format!("This enum variant is not {}::{}!",#enum_name,#variant_name)),
                            }
                        }
                    ));
                } else if let syn::Fields::Unnamed(fields_unamed) = &variant.fields {
                    let fields = &fields_unamed.unnamed;
                    if fields.len() > 1 {
                        continue;
                    }
                    let field_ty = &fields[0].ty;
                    get_variant_functions.extend(quote!(
                        pub fn #function_name(&self) -> Result<&#field_ty,String>{
                            match self {
                                #enum_ident::#variant_ident(data)=>Ok(data),
                                _=>Err(format!("This enum variant is not {}::{}!",#enum_name,#variant_name)),
                            }
                        }
                        pub fn #function_mut_name(&mut self) -> Result<&mut #field_ty,String>{
                            match self {
                                #enum_ident::#variant_ident(data)=>Ok(data),
                                _=>Err(format!("This enum variant is not {}::{}!",#enum_name,#variant_name)),
                            }
                        }
                    ));
                } else if let syn::Fields::Unit = &variant.fields {
                    continue;
                }
            }
        }
        _ => return derive_error!("GetVariant only works on enum!"),
    };

    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();
    let expanded = quote! {
        impl #impl_generics #enum_ident #ty_generics #where_clause {
            #get_variant_functions
        }
    };
    TokenStream::from(expanded)
}
