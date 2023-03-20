use bae::FromAttributes;
use syn;

#[derive(Default, FromAttributes, Debug)]
pub struct Attr {
    pub save: Option<()>,
    pub replicated: Option<()>,
}

impl Attr {
    pub fn should_save(&self) -> bool {
        self.save.is_some()
    }

    pub fn should_replicate(&self) -> bool {
        self.replicated.is_some()
    }
}
