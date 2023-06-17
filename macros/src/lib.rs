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

struct Object {
    prefix: String,
    name: String,
    fields: Option<&'static Fields>,
    ty: &'static Type,
}

fn process_object(obj: &Object) -> (TokenStream, Option<TokenStream>) {
    // match obj.ty {
    //     Type::Path(TypePath { .. }) => {
    //         let name = &obj.name;
    //         let prefix = &obj.prefix;
    //         let ty = &obj.ty;

    //         let full_name = format!("{}_{}", prefix, name).trim().to_string();

    //         let parse_fn_name = format!("parse_{}", full_name);

    //         let mut fields = vec![];
    //         let mut field_names = vec![];

    //         let mut total_implementations = vec![];

    //         // if let syn::Data::Struct(s) = obj.data.clone() {
    //         //     if let syn::Fields::Named(FieldsNamed { named, .. }) = s.fields {
    //         for field in obj.fields.unwrap() {
    //             let field_name = field.ident.unwrap();
    //             let field_type = field.ty;

    //             match field_type {
    //                 Type::Path(TypePath { qself, .. }) => {
    //                     let input = parse_macro_input!(s);
    //                     let DeriveInput { ident, data, .. } = &input;

    //                     let (expanded, implementation) = process_object(&Object {
    //                         prefix: full_name.clone(),
    //                         name: field_name.to_string(),
    //                         fields: field.to_owned(),
    //                         ty: &field_type,
    //                     });

    //                     fields.push(expanded);

    //                     if let Some(implementation) = implementation {
    //                         total_implementations.push(implementation);
    //                     }
    //                 }
    //                 _ => {}
    //             }

    //             // fields.push(quote! {
    //             //     let #field_name = match std::env::var(format!("{}{}", prefix, stringify!(#field_name).to_uppercase())) {
    //             //         Ok(val) => val.parse::<#field_type>().expect("Failed to parse environment variable"),
    //             //         Err(_) => #field_type::default(),
    //             //     };
    //             // });

    //             // field_names.push(quote! {
    //             //     #field_name
    //             // });
    //         }
    //         //     }
    //         // }

    //         let implementation = quote! {
    //             fn #parse_fn_name() -> #ty {
    //                 #(#fields)*

    //                 #ty {
    //                     #(#field_names),*
    //                 }
    //             }
    //         };

    //         let expanded = quote! {
    //             let #name = match std::env::var(format!("{}{}", #prefix, stringify!(#name).to_uppercase())) {
    //                 Ok(val) => val.parse::<#ty>().unwrap_or_else(|_| #parse_fn_name()),
    //                 Err(_) => #parse_fn_name(),
    //             };
    //         };

    //         (expanded.into(), Some(implementation.into()))
    //     }
    //     _ => {
    //         let name = &obj.name;
    //         let prefix = &obj.prefix;
    //         let ty = &obj.ty;

    //         let expanded = quote! {
    //             let #name = match std::env::var(format!("{}{}", #prefix, stringify!(#name).to_uppercase())) {
    //                 Ok(val) => val.parse::<#ty>().unwrap_or_else(|_| #ty::default()),
    //                 Err(_) => #ty::default(),
    //             };
    //         };

    //         (expanded.into(), None)
    //     }
    // }
    todo!();
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
                todo!();
            }
        }
    }
    .into()
}
