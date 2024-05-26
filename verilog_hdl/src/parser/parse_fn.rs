use super::{
    parse_block::{parse_block_main, Block},
    Item,
};
use syn::{punctuated::Punctuated, token::Comma};

pub struct Function {
    pub fn_name: String,
    pub args: Vec<String>,
    pub block: Block,
}

pub fn parse_items(rust_ast_items: Vec<syn::Item>) -> Vec<Item> {
    let mut res = Vec::new();

    for item in rust_ast_items {
        match item {
            syn::Item::Const(_) => todo!(),
            syn::Item::Enum(_) => todo!(),
            syn::Item::ExternCrate(_) => todo!(),
            syn::Item::Fn(f) => res.push(Item::Fn(parse_fn(f))),
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

    res
}

pub fn parse_fn(f: syn::ItemFn) -> Function {
    Function {
        fn_name: f.sig.ident.to_string(),
        args: parse_args(f.sig.inputs.clone()),
        block: parse_block_main(&f),
    }
}

pub fn parse_args(inputs: Punctuated<syn::FnArg, Comma>) -> Vec<String> {
    let mut args_str = Vec::new();

    for panc in inputs {
        match panc {
            syn::FnArg::Receiver(_) => todo!(),
            syn::FnArg::Typed(pad_typed) => match *pad_typed.pat {
                syn::Pat::Const(_) => todo!(),
                syn::Pat::Ident(pat_ident) => args_str.push(pat_ident.ident.to_string()),
                syn::Pat::Lit(_) => todo!(),
                syn::Pat::Macro(_) => todo!(),
                syn::Pat::Or(_) => todo!(),
                syn::Pat::Paren(_) => todo!(),
                syn::Pat::Path(_) => todo!(),
                syn::Pat::Range(_) => todo!(),
                syn::Pat::Reference(_) => todo!(),
                syn::Pat::Rest(_) => todo!(),
                syn::Pat::Slice(_) => todo!(),
                syn::Pat::Struct(_) => todo!(),
                syn::Pat::Tuple(_) => todo!(),
                syn::Pat::TupleStruct(_) => todo!(),
                syn::Pat::Type(_) => todo!(),
                syn::Pat::Verbatim(_) => todo!(),
                syn::Pat::Wild(_) => todo!(),
                _ => todo!(),
            },
        }
    }

    args_str
}
