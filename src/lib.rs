mod test;
mod traits;

// pub fn add(left: usize, right: usize) -> usize {
//     left + right
// }

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, FieldsNamed};

#[proc_macro_derive(Settings)]
pub fn derive_answer_fn(item: TokenStream) -> TokenStream {
    let DeriveInput { ident, data, .. } = parse_macro_input!(item);

    // println!("{}", data);

    let mut fields_implement = TokenStream::new();

    if let syn::Data::Struct(s) = data {
        if let syn::Fields::Named(FieldsNamed { named, .. }) = s.fields {
            for field in named {
                // println!("{}", field.ident.unwrap());
                let field_name = field.ident.unwrap();
                fields_implement.extend::<TokenStream>(
                    quote! {
                        #field_name: String,
                    }
                    .into(),
                )
            }
        }
    }

    println!("{}", fields_implement);

    quote! {
        impl #ident {
            fn load_from_env() -> Self {
                // #ident {

                // }
                todo!();
                // let mut settings = Self::default();
                // settings.load_from_env();
                // settings
            }
        }
    }
    .into()
}

// impl Settings {
//     pub fn parse() -> Settings {
//         let host = std::env::var("host").unwrap_or("".into());
//         let port = std::env::var("port")
//             .unwrap_or("8080".into())
//             .parse::<u16>()
//             .unwrap_or(8080);

//         Settings { host, port }
//     }
// }
