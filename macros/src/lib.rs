// mod test;
// mod traits;

use std::any::Any;

use proc_macro::TokenStream;
use quote::{quote, spanned::Spanned, ToTokens};
use syn::{parse_macro_input, DeriveInput, FieldsNamed};
use traits::FromEnv;

#[proc_macro_derive(Settings, attributes(settings))]
pub fn derive_settings(item: TokenStream) -> TokenStream {
    let DeriveInput { ident, data, .. } = parse_macro_input!(item);
    let mut field_deserializers = vec![];
    let mut field_names = vec![];

    if let syn::Data::Struct(s) = data {
        if let syn::Fields::Named(FieldsNamed { named, .. }) = s.fields {
            for field in named {
                let field_name = field.clone().ident.unwrap();
                let field_type = &field.ty;
                field_deserializers.push(quote! {
                    let #field_name = match std::env::var(format!("{}{}", prefix, stringify!(#field_name).to_uppercase())) {
                        Ok(val) => val.parse::<#field_type>().expect("Failed to parse environment variable"),
                        Err(_) => #field_type::default(),
                    };
                });
                field_names.push(field_name);
            }
        }
    }

    let expanded = quote! {
        impl #ident {
            fn load_from_env() -> Self {
                let prefix = std::env::var("PREFIX").unwrap_or_default();
                #(#field_deserializers)*
                Self {
                    #(#field_names),*
                }
            }
        }
    };

    TokenStream::from(expanded)
}
