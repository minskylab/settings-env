use darling::FromDeriveInput;
use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse_macro_input, DeriveInput, FieldsNamed};

#[derive(FromDeriveInput, Default)]
#[darling(default, attributes(settings))]
struct Opts {
    env: Option<String>,
}

#[proc_macro_derive(Settings, attributes(settings))]
pub fn derive_settings(item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item);
    let DeriveInput { ident, data, .. } = &input;
    let opts = Opts::from_derive_input(&input).expect("Wrong options");

    let mut field_deserializers = vec![];
    let mut field_names = vec![];

    if let syn::Data::Struct(s) = data {
        if let syn::Fields::Named(FieldsNamed { named, .. }) = &s.fields {
            for field in named {
                let field_name = field.clone().ident.unwrap();
                let field_type = &field.ty;
                field_deserializers.push(quote! {
                    let #field_name = match std::env::var(format!("{}{}", prefix, stringify!(#field_name).to_uppercase())) {
                        Ok(val) => val.parse::<#field_type>().unwrap_or_else(|_| #field_type::load_from_env()),
                        Err(_) => #field_type::default(),
                    };
                });
                field_names.push(field_name);
            }
        }
    }

    // Si se proporcionó la opción de env, generamos un método adicional.
    let env_method = match &opts.env {
        Some(env) => quote! {
            fn load_from_env_var() -> Option<String> {
                std::env::var(#env).ok()
            }
        },
        None => quote! {},
    };

    let expanded = quote! {
        impl #ident {
            #env_method

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
