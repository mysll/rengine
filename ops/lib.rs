mod attributes;
mod object;

use proc_macro::TokenStream;
use proc_macro2::Ident;
use quote::quote;
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
        let add_attr = vec![
            quote! {__model: re_object::game_model::Model},
            quote! {__go: re_object::MutObjectPtr},
        ];
        //let add_attr: Vec<proc_macro2::TokenStream> = Vec::new();
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

#[proc_macro_derive(Entity, attributes(attr))]
pub fn entity_builder(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = parse_macro_input!(input);
    let mut attrs: Vec<Ident> = Vec::new();
    let mut fn_attrs: Vec<proc_macro2::TokenStream> = Vec::new();
    let mut save_attrs: Vec<Ident> = Vec::new();
    let mut rep_attrs: Vec<Ident> = Vec::new();
    let mut match_any_set: Vec<proc_macro2::TokenStream> = Vec::new();
    let mut match_any_get: Vec<proc_macro2::TokenStream> = Vec::new();
    let mut match_attr_set: Vec<proc_macro2::TokenStream> = Vec::new();
    let mut match_attr_get: Vec<proc_macro2::TokenStream> = Vec::new();

    let ident = object::parse_token(
        ast,
        &mut attrs,
        &mut fn_attrs,
        &mut save_attrs,
        &mut rep_attrs,
        &mut match_any_set,
        &mut match_any_get,
        &mut match_attr_set,
        &mut match_attr_get,
    );

    let entity_token = object::make_entity(
        &ident,
        &attrs,
        &fn_attrs,
        &save_attrs,
        &rep_attrs,
        &match_any_set,
        &match_any_get,
        &mut match_attr_set,
        &mut match_attr_get,
    );
    let object_token = object::make_object(&ident);

    let output = quote! {
        #entity_token
        #object_token
        inventory::submit! {
            re_object::registry::ObjectInitializer::register_entity(stringify!(#ident), || std::rc::Rc::new(std::cell::RefCell::new(#ident::new())))
        }
    };
    output.into()
}
