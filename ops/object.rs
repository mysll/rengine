use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};
use syn::DeriveInput;

use crate::attributes::Attr;

pub fn parse_token(
    ast: DeriveInput,
    attrs: &mut Vec<Ident>,
    fn_attrs: &mut Vec<TokenStream>,
    save_attrs: &mut Vec<Ident>,
    rep_attrs: &mut Vec<Ident>,
    match_any_set: &mut Vec<TokenStream>,
    match_any_get: &mut Vec<TokenStream>,
    match_attr_set: &mut Vec<TokenStream>,
    match_attr_get: &mut Vec<TokenStream>,
) -> Ident {
    let DeriveInput { ident, .. } = ast;
    if let syn::Data::Struct(syn::DataStruct { fields, .. }) = ast.data {
        let mut index: u32 = 0;
        for field in fields {
            let ident_field = field.ident.unwrap();
            let ty = &field.ty;
            if let Ok(attr) = Attr::from_attributes(&field.attrs) {
                let get = format_ident!("get_{}", ident_field);
                let set = format_ident!("set_{}", ident_field);
                let set_any = format_ident!("set_{}_any", ident_field);
                attrs.push(ident_field.clone());
                let fp = quote! {
                    pub fn #get<'a>(&'a self) -> &'a #ty{
                        &self.#ident_field
                    }
                    pub fn #set(&mut self, val:#ty) {
                        if self.#ident_field == val {
                            return;
                        }
                        let old = std::mem::replace(&mut self.#ident_field, val);
                        self.change_attr(#index, &old);
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
                    #index => {
                        self.#set_any(v)
                    }
                });
                match_any_get.push(quote! {
                    #index => {
                        Some(self.#get())
                    }
                });
                match_attr_set.push(quote! {
                    stringify!(#ident_field) => {
                        self.#set_any(v)
                    }
                });
                match_attr_get.push(quote! {
                    stringify!(#ident_field) => {
                        Some(self.#get())
                    }
                });
                index += 1;
            }
        }
    }
    ident
}

pub fn make_entity(
    ident: &Ident,
    attrs: &Vec<Ident>,
    fn_attrs: &Vec<TokenStream>,
    save_attrs: &Vec<Ident>,
    rep_attrs: &Vec<Ident>,
    match_any_set: &Vec<TokenStream>,
    match_any_get: &Vec<TokenStream>,
    match_attr_set: &mut Vec<TokenStream>,
    match_attr_get: &mut Vec<TokenStream>,
) -> TokenStream {
    quote! {
        impl #ident {
            pub fn new() -> Self {
                let mut d = Self::default();
                let attrs:Vec<&'static str>= vec![ #(stringify!(#attrs)),* ];
                let saves:Vec<&'static str> = vec![ #(stringify!(#save_attrs)),* ];
                let reps:Vec<&'static str> = vec![ #(stringify!(#rep_attrs)),* ];
                d.__model = re_object::game_model::Model::new(stringify!(#ident), attrs, saves, reps);
                d
            }
            pub fn ClassName() -> &'static str {
                stringify!(#ident)
            }
            pub fn change_attr(&mut self, index:u32, old:&dyn std::any::Any) {
                unsafe{
                    (*self.__go.0).change_attr(index, old);
                }
            }
            pub fn set_attr_by_index(&mut self, att: u32, v :&dyn std::any::Any) -> bool {
                match att {
                    #(#match_any_set) *
                    _ => false
                }
            }
            pub fn get_attr_by_index<'a>(&'a self, att: u32) ->Option<&'a dyn std::any::Any> {
                match att {
                    #(#match_any_get) *
                    _ => None
                }
            }
            pub fn set_attr(&mut self, att: &str, v :&dyn std::any::Any) -> bool {
                match att {
                    #(#match_attr_set) *
                    _ => false
                }
            }
            pub fn get_attr<'a>(&'a self, att: &str) ->Option<&'a dyn std::any::Any> {
                match att {
                    #(#match_attr_get) *
                    _ => None
                }
            }
            #(#fn_attrs) *
        }
        /*
        impl AsRef<re_entity::entity::Entity> for #ident {
            fn as_ref(&self) -> &re_entity::entity::Entity {
                &self.__internal
            }
        }

        impl AsMut<re_entity::entity::Entity> for #ident {
            fn as_mut(&mut self) -> &mut re_entity::entity::Entity {
                &mut self.__internal
            }
        }
        */

    }
}

pub fn make_object(ident: &Ident) -> TokenStream {
    quote! {
        impl re_object::game_model::GameModel for #ident {
            fn get_model(&self) -> re_object::game_model::Model {
                self.__model.clone()
            }
            fn set_gameobj(&mut self, go: *mut re_object::object::Object){
                self.__go=re_object::MutObjectPtr(go);
            }
            fn get_attr_by_name<'a>(&'a self, attr: &str) -> Option<&'a dyn std::any::Any> {
                self.get_attr(attr)
            }
            fn set_attr_by_name(&mut self, attr: &str, val: &dyn std::any::Any) -> bool {
                self.set_attr(attr, val)
            }
            fn get_attr_by_index<'a>(&'a self, index: u32) -> Option<&'a dyn std::any::Any> {
                self.get_attr_by_index(index)
            }
            fn set_attr_by_index(&mut self, index: u32, val: &dyn std::any::Any) -> bool {
                self.set_attr_by_index(index, val)
            }
            fn get_any<'a>(&'a self) -> &'a dyn std::any::Any {
                self
            }
            fn get_mut_any<'a>(&'a mut self) -> &'a mut dyn std::any::Any {
                self
            }
        }
    }
}
