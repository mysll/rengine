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
                        self.__internal.change_attr(#index, &old);
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
            pub fn ClassName() -> &'static str {
                stringify!(#ident)
            }
            pub fn set_attr(&mut self, att: u32, v :&dyn std::any::Any) -> bool {
                match att {
                    #(#match_any_set) *
                    _ => false
                }
            }
            pub fn get_attr<'a>(&'a self, att: u32) ->Option<&'a dyn std::any::Any> {
                match att {
                    #(#match_any_get) *
                    _ => None
                }
            }
            #(#fn_attrs) *
        }

        impl AsRef<dyn re_entity::entity::Entity> for #ident {
            fn as_ref(&self) -> &(dyn re_entity::entity::Entity + 'static) {
                &self.__internal
            }
        }

        impl AsMut<dyn re_entity::entity::Entity> for #ident {
            fn as_mut(&mut self) -> &mut (dyn re_entity::entity::Entity + 'static) {
                &mut self.__internal
            }
        }

    }
}

pub fn make_object(ident: &Ident) -> TokenStream {
    quote! {
        impl re_entity::entity::Object for #ident {
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
            fn get_attr_by_index<'a>(&'a self, index: u32) -> Option<&'a dyn std::any::Any> {
                self.get_attr(index)
            }
            fn set_attr_by_index(&mut self, index: u32, val: &dyn std::any::Any) -> bool {
                self.set_attr(index, val)
            }
            fn entity_ref<'a>(&'a self) -> &'a dyn re_entity::entity::Entity {
                &self.__internal
            }
            fn entity_mut<'a>(&'a mut self) -> &'a mut dyn re_entity::entity::Entity{
                &mut self.__internal
            }
        }
    }
}
