mod attributes;

use inflector::cases::pascalcase::to_pascal_case;
use proc_macro::TokenStream;
use proc_macro2::Ident;
use quote::{format_ident, quote};
use syn::{
    parse::{self, Parser},
    parse_macro_input, DeriveInput, ItemStruct,
};

use crate::attributes::Attr;

#[proc_macro_attribute]
pub fn def_entity(args: TokenStream, input: TokenStream) -> TokenStream {
    let mut item_struct = parse_macro_input!(input as ItemStruct);
    let _ = parse_macro_input!(args as parse::Nothing);
    if let syn::Fields::Named(ref mut fields) = item_struct.fields {
        let add_attr = vec![
            quote! {__attrs: Vec<&'static str>},
            quote! {__saves_index: Vec<u32>},
            quote! {__reps_index: Vec<u32>},
            quote! {__saves: Vec<&'static str>},
            quote! {__reps: Vec<&'static str>},
            quote! {__index: std::collections::HashMap<&'static str,u32>},
            quote! {__dirty: bool},
            quote! {__modify_attrs: Vec<u32>},
        ];
        for att in add_attr {
            fields
                .named
                .push(syn::Field::parse_named.parse2(att).unwrap());
        }
    }

    return quote! {
        #[derive(Default, re_ops::Entity)]
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
    let DeriveInput { ident, .. } = ast;

    let mut attrs = Vec::new();
    let mut attrs_up = Vec::new();
    let mut fn_attrs = Vec::new();
    let mut save_attrs = Vec::new();
    let mut rep_attrs = Vec::new();
    let mut match_any_set = Vec::new();
    let mut match_any_get = Vec::new();
    let ident_enum = format_ident!("{}Attrs", ident);
    if let syn::Data::Struct(syn::DataStruct { fields, .. }) = ast.data {
        for field in fields {
            let ident_field = field.ident.unwrap();
            let ty = &field.ty;
            if let Ok(attr) = Attr::from_attributes(&field.attrs) {
                let get = format_ident!("get_{}", ident_field);
                let set = format_ident!("set_{}", ident_field);
                let set_any = format_ident!("set_{}_any", ident_field);
                let get_enum = format_ident!("get_{}_enum", ident_field);
                let ident_field_enum = to_pascal_case(&ident_field.to_string());
                let attr_enum = Ident::new(&ident_field_enum, ident_field.span());
                attrs_up.push(attr_enum.clone());
                attrs.push(ident_field.clone());
                let fp = quote! {
                    pub fn #get<'a>(&'a self) -> &'a #ty{
                        &self.#ident_field
                    }
                    pub fn #set(&mut self, val:#ty) {
                        self.#ident_field = val
                    }
                    pub fn #get_enum(&self) -> #ident_enum{
                        #ident_enum::#attr_enum
                    }
                    pub fn #set_any(&mut self, val:&dyn std::any::Any) -> bool {
                        match val.downcast_ref::<#ty>() {
                            Some(v) => {
                                self.#set(v.clone());
                                true
                            }
                            None => false,
                        }
                    }
                };

                fn_attrs.push(fp);

                if attr.should_save() {
                    save_attrs.push(ident_field.clone());
                }
                if attr.should_replicate() {
                    rep_attrs.push(ident_field.clone());
                }

                match_any_set.push(quote! {
                    #ident_enum::#attr_enum => {
                        self.#set_any(v)
                    }
                });
                match_any_get.push(quote! {
                    #ident_enum::#attr_enum => {
                        self.#get()
                    }
                });
            }
        }
    }

    let output = quote! {

        #[derive(re_ops::NumEnum)]
        pub enum #ident_enum {
            #(#attrs_up),*
        }


        impl #ident {
            pub fn new() -> Self {
                let mut d = Self::default();
                d.__attrs = vec![ #(stringify!(#attrs)),* ];
                d.__index = std::collections::HashMap::new();
                d.__attrs.iter()
                .enumerate()
                .for_each(|(i, attr)| {d.__index.insert(attr, i as u32);});

                d.__saves_index = Vec::new();
                d.__saves = vec![ #(stringify!(#save_attrs)),* ];
                d.__saves.iter().enumerate().for_each(|(_, attr)| {d.__saves_index.push(d.__index[attr])});

                d.__reps_index = Vec::new();
                d.__reps = vec![ #(stringify!(#rep_attrs)),* ];
                d.__reps.iter().enumerate().for_each(|(_, attr)| {d.__reps_index.push(d.__index[attr])});

                d
            }
            pub fn set_attr(&mut self, att: #ident_enum, v :&dyn std::any::Any) -> bool {
                match att {
                    #(#match_any_set) *
                }
            }
            pub fn get_attr<'a>(&'a self, att: #ident_enum) ->&'a dyn std::any::Any {
                match att {
                    #(#match_any_get) *
                }
            }
            #(#fn_attrs) *
        }

        use re_entity::entity::Object;
        impl Object for #ident {
            fn get_attrs<'a>(&'a self) -> &'a Vec<&str> {
                &self.__attrs
            }
            fn save_attrs<'a>(&'a self) -> &'a Vec<&str>{
                &self.__saves
            }
            fn rep_attrs<'a>(&'a self) -> &'a Vec<&str>{
                &self.__reps
            }
            fn save_attrs_index(&self) -> &Vec<u32> {
                &self.__saves_index
            }
            fn rep_attrs_index(&self) -> &Vec<u32> {
                &self.__reps_index
            }
            fn get_attr_count(&self) -> u32 {
                self.__attrs.len() as u32
            }
            fn get_attr_by_name<'a>(&'a self, attr: &str) -> Option<&'a dyn std::any::Any> {
                match self.__index.get(attr) {
                    Some(&i) => self.get_attr_by_index(i),
                    None=> None
                }
            }
            fn set_attr_by_name(&mut self, attr: &str, val: &dyn std::any::Any) -> bool {
                match self.__index.get(attr) {
                    Some(&i) => self.set_attr_by_index(i, val),
                    None=> false
                }
            }
            fn get_attr_name<'a>(&'a self, index: u32) -> Option<&'a str> {
                match self.__attrs.get(index as usize) {
                    Some(attr) => Some(attr),
                    None=>None,
                }
            }
            fn get_attr_index(&self, attr: &str) -> Option<u32> {
                match self.__index.get(attr) {
                    Some(&i) => Some(i),
                    None=> None
                }
            }
            fn get_attr_by_index<'a>(&'a self, index: u32) -> Option<&'a dyn std::any::Any> {
                if let Some(att) = #ident_enum::get_attr_enum_by_index(index) {
                    Some(self.get_attr(att))
                } else {
                    None
                }
            }
            fn set_attr_by_index(&mut self, index: u32, val: &dyn std::any::Any) -> bool {
                if let Some(att) = #ident_enum::get_attr_enum_by_index(index) {
                    self.set_attr(att, val)
                } else {
                    false
                }
            }
        }

    };
    output.into()
}
