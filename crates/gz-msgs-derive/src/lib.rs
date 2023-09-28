use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;

#[proc_macro_derive(GzMessage, attributes(ser_type_info))]
pub fn gz_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as syn::DeriveInput);
    let type_name = &input.ident;

    match &input.data {
        syn::Data::Struct(_) => quote! {
            impl crate::GzMessage for #type_name {
                const GZ_TYPE_NAME: &'static str = concat!("gz.msgs.", stringify!(#type_name));
            }
        }
        .into(),
        _ => panic!("Enums or Unions are not supported"),
    }
}

#[proc_macro_derive(IgnMessage, attributes(ser_type_info))]
pub fn ign_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as syn::DeriveInput);
    let type_name = &input.ident;

    match &input.data {
        syn::Data::Struct(_) => quote! {
            impl crate::GzMessage for #type_name {
                const GZ_TYPE_NAME: &'static str = concat!("ignition.msgs.", stringify!(#type_name));
            }
        }
        .into(),
        _ => panic!("Enums or Unions are not supported"),
    }
}
