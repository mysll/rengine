mod attributes;
mod object;

use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use quote::{format_ident, quote};
use syn::{
    parse::{self, Parser},
    parse_macro_input, DeriveInput, ItemStruct,
};

#[proc_macro_attribute]
pub fn def_entity(args: TokenStream, input: TokenStream) -> TokenStream {
    let mut item_struct = parse_macro_input!(input as ItemStruct);
    let _ = parse_macro_input!(args as parse::Nothing);
    if let syn::Fields::Named(ref mut fields) = item_struct.fields {
        // 插入一个占位属性
        fields.named.insert(
            0,
            syn::Field::parse_named
                .parse2(quote! {#[attr()]none: ()})
                .unwrap(),
        );
        let add_attr = vec![quote! {__internal: re_entity::entity::EntityInfo}];
        for att in add_attr {
            fields
                .named
                .push(syn::Field::parse_named.parse2(att).unwrap());
        }
    }
    return quote! {
        #[derive(Default, Debug, re_ops::Entity)]
        #[allow(dead_code)]
        #item_struct
    }
    .into();
}

#[proc_macro_derive(NumEnum)]
pub fn num_enum_builder(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = parse_macro_input!(input);
    let DeriveInput { ident, .. } = ast;
    let mut enums = Vec::new();
    if let syn::Data::Enum(syn::DataEnum { variants, .. }) = ast.data {
        for variant in variants {
            let enum_ident = variant.ident;
            enums.push(enum_ident);
        }
    };
    let output = quote! {
        impl #ident {
            pub fn get_attr_enum_by_index(i:u32) -> Option<Self> {
                match i {
                    #(x if x == Self::#enums as u32 => Some(Self::#enums),) *
                    _ => None,
                }
            }
        }
    };
    output.into()
}

#[proc_macro_derive(Entity, attributes(attr))]
pub fn entity_builder(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = parse_macro_input!(input);
    let mut attrs: Vec<Ident> = Vec::new();
    let mut fn_attrs: Vec<proc_macro2::TokenStream> = Vec::new();
    let mut save_attrs: Vec<Ident> = Vec::new();
    let mut rep_attrs: Vec<Ident> = Vec::new();
    let mut match_any_set: Vec<proc_macro2::TokenStream> = Vec::new();
    let mut match_any_get: Vec<proc_macro2::TokenStream> = Vec::new();

    let ident = object::parse_token(
        ast,
        &mut attrs,
        &mut fn_attrs,
        &mut save_attrs,
        &mut rep_attrs,
        &mut match_any_set,
        &mut match_any_get,
    );

    let entity_token = object::make_entity(
        &ident,
        &attrs,
        &fn_attrs,
        &save_attrs,
        &rep_attrs,
        &match_any_set,
        &match_any_get,
    );
    let object_token = object::make_object(&ident);

    let output = quote! {
        #entity_token
        #object_token
        inventory::submit! {
            re_entity::entity::ObjectInitializer::register_entity(stringify!(#ident), || Box::new(#ident::new()))
        }
    };
    output.into()
}
