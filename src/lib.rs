extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;
use syn::Data;
use syn::DataStruct;
use syn::DeriveInput;
use syn::Fields;

#[proc_macro_derive(DeserializeFromArray)]
pub fn deserialize_from_array(input: TokenStream) -> TokenStream {
    let DeriveInput { ident, data, .. } = parse_macro_input!(input as DeriveInput);
    let struct_name = &ident;

    let fields = match &data {
        Data::Struct(DataStruct {
            fields: Fields::Named(fields),
            ..
        }) => &fields.named,
        _ => panic!("expected a struct with named fields"),
    };
    let count = fields.len();
    let field_name = fields.iter().map(|field| &field.ident);

    let gen = quote! {
        impl<'de> serde::Deserialize<'de> for #struct_name {
            fn deserialize<D>(deserializer: D) -> Result<#struct_name, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                let mut values = Vec::<serde_json::Value>::deserialize(deserializer)?;
                if #count != values.len() {
                    let error = format!("The number of elements in the list is not {}", #count);
                    return Err(serde::de::Error::custom(error));
                }
                Ok(#struct_name {
                    #(
                        #field_name: serde_json::from_value(values.remove(0)).map_err(serde::de::Error::custom)?,
                    )*
                })
            }
        }
    };
    gen.into()
}

#[proc_macro_derive(SerializeToArray)]
pub fn serialize_to_array(input: TokenStream) -> TokenStream {
    let DeriveInput { ident, data, .. } = parse_macro_input!(input as DeriveInput);
    let struct_name = &ident;

    let fields = match &data {
        Data::Struct(DataStruct {
            fields: Fields::Named(fields),
            ..
        }) => &fields.named,
        _ => panic!("expected a struct with named fields"),
    };
    let count = fields.len();
    let field_name = fields.iter().map(|field| &field.ident);

    let gen = quote! {
        impl serde::Serialize for #struct_name {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                let mut values = Vec::with_capacity(#count);
                #(
                    let value = serde_json::to_value(&self.#field_name).map_err(serde::ser::Error::custom)?;
                    values.push(value);
                )*
                values.serialize(serializer)
            }
        }
    };
    gen.into()
}
