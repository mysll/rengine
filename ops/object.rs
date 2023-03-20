use inflector::cases::pascalcase::to_pascal_case;
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};
use syn::DeriveInput;

use crate::attributes::Attr;

pub fn parse_token(
    ast: DeriveInput,
    attrs: &mut Vec<Ident>,
    attrs_up: &mut Vec<Ident>,
    fn_attrs: &mut Vec<TokenStream>,
    save_attrs: &mut Vec<Ident>,
    rep_attrs: &mut Vec<Ident>,
    match_any_set: &mut Vec<TokenStream>,
    match_any_get: &mut Vec<TokenStream>,
) -> (Ident, Ident) {
    let DeriveInput { ident, .. } = ast;
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
                        if self.#ident_field == val {
                            return;
                        }
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
    (ident, ident_enum)
}

pub fn make_entity(
    ident: &Ident,
    ident_enum: &Ident,
    attrs: &Vec<Ident>,
    fn_attrs: &Vec<TokenStream>,
    save_attrs: &Vec<Ident>,
    rep_attrs: &Vec<Ident>,
    match_any_set: &Vec<TokenStream>,
    match_any_get: &Vec<TokenStream>,
) -> TokenStream {
    quote! {
        impl #ident {
            pub fn new() -> Self {
                let mut d = Self::default();
                let attrs = vec![ #(stringify!(#attrs)),* ];
                let saves = vec![ #(stringify!(#save_attrs)),* ];
                let reps = vec![ #(stringify!(#rep_attrs)),* ];
                d.__internal.init(attrs, saves, reps);
                d
            }
            pub fn dirty(&self) -> bool {
                self.__internal.dirty()
            }
            pub fn clear_dirty(&mut self) {
                self.__internal.clear_dirty()
            }
            pub fn modify(&self) -> bool {
                self.__internal.modify()
            }
            pub fn clear_modify(&mut self) {
                self.__internal.clear_modify()
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
    }
}

pub fn make_object(ident: &Ident, ident_enum: &Ident) -> TokenStream {
    quote! {
        use re_entity::entity::Object;
        impl Object for #ident {
            fn get_attrs<'a>(&'a self) -> &'a Vec<&str> {
                &self.__internal.attrs
            }
            fn save_attrs<'a>(&'a self) -> &'a Vec<&str>{
                &self.__internal.saves
            }
            fn rep_attrs<'a>(&'a self) -> &'a Vec<&str>{
                &self.__internal.reps
            }
            fn save_attrs_index(&self) -> &Vec<u32> {
                &self.__internal.saves_index
            }
            fn rep_attrs_index(&self) -> &Vec<u32> {
                &self.__internal.reps_index
            }
            fn get_attr_count(&self) -> u32 {
                self.__internal.attr_count()
            }
            fn get_attr_by_name<'a>(&'a self, attr: &str) -> Option<&'a dyn std::any::Any> {
                match self.__internal.get_attr_index(attr) {
                    Some(i) => self.get_attr_by_index(i),
                    None=> None
                }
            }
            fn set_attr_by_name(&mut self, attr: &str, val: &dyn std::any::Any) -> bool {
                match self.__internal.get_attr_index(attr) {
                    Some(i) => self.set_attr_by_index(i, val),
                    None=> false
                }
            }
            fn get_attr_name<'a>(&'a self, index: u32) -> Option<&'a str> {
                self.__internal.get_attr_name(index)
            }
            fn get_attr_index(&self, attr: &str) -> Option<u32> {
                self.__internal.get_attr_index(attr)
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
    }
}