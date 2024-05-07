use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Model, attributes(model))]
pub fn derive_model(input: TokenStream) -> TokenStream {
    let DeriveInput { ident, .. } = parse_macro_input!(input as DeriveInput);
    quote! {
        impl gnify::model::Model for #ident {
            type ID = ::ulid::Ulid;
            const NAME: &'static str = stringify!(#ident);
        }
    }
    .into()
}

#[proc_macro_derive(Identifiable)]
pub fn derive_identifiable(input: TokenStream) -> TokenStream {
    let DeriveInput { ident, .. } = parse_macro_input!(input);
    quote! {
        impl gnify::model::Identifiable for #ident {
            type ID = ::ulid::Ulid;
            const NAME: &'static str = stringify!(#ident);
        }
    }
    .into()
}

