pub mod block;
pub mod parse_fn;

use crate::generate_main::Serializer;

impl Serializer {
    pub(crate) fn parse_items(&mut self, rust_ast_items: Vec<syn::Item>) {
        for item in rust_ast_items {
            match item {
                syn::Item::Const(_) => todo!(),
                syn::Item::Enum(_) => todo!(),
                syn::Item::ExternCrate(_) => todo!(),
                syn::Item::Fn(f) => self.parse_fn(f),
                syn::Item::ForeignMod(_) => todo!(),
                syn::Item::Impl(_) => todo!(),
                syn::Item::Macro(_) => todo!(),
                syn::Item::Mod(_) => todo!(),
                syn::Item::Static(_) => todo!(),
                syn::Item::Struct(_) => todo!(),
                syn::Item::Trait(_) => todo!(),
                syn::Item::TraitAlias(_) => todo!(),
                syn::Item::Type(_) => todo!(),
                syn::Item::Union(_) => todo!(),
                syn::Item::Use(_) => todo!(),
                syn::Item::Verbatim(_) => todo!(),
                _ => todo!(),
            }
        }
    }
}
