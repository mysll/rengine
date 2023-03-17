use bae::FromAttributes;
use syn;

#[derive(Default, FromAttributes, Debug)]
pub struct Attr {
    pub save: Option<syn::LitBool>,
    pub replicated: Option<syn::LitBool>,
}

impl Attr {
    pub fn should_save(&self) -> bool {
        match &self.save {
            Some(save) => save.value,
            None => false,
        }
    }

    pub fn should_replicate(&self) -> bool {
        match &self.replicated {
            Some(rep) => rep.value,
            None => false,
        }
    }
}
