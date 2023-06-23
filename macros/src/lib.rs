use darling::FromDeriveInput;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields, FieldsNamed, Type, TypePath};

#[derive(FromDeriveInput, Default)]
#[darling(default, attributes(settings))]
struct Opts {
    env: Option<String>,
    prefix: Option<String>,
}

#[proc_macro_derive(Settings, attributes(settings))]
pub fn derive_settings(item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item);
    let DeriveInput { ident, data, .. } = &input;
    let opts = Opts::from_derive_input(&input).expect("Wrong options");

    let root_prefix = opts.prefix.unwrap_or("".into());

    println!("{:?}", input);

    quote! {
        use settings_env::errors::SettingsError;

        impl #ident {
            fn load() -> Result<Self, SettingsError> {
                serde_json::to_value();
                todo!();
            }
        }
    }
    .into()
}
