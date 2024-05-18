pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn parse_file(path: impl AsRef<std::path::Path>) -> std::io::Result<syn::File> {
    let code = std::fs::read_to_string(path)?;
    let ast = syn::parse_file(&code).unwrap();
    Ok(ast)
}

pub fn parse_fn(ast: &syn::File) -> &Box<syn::Block> {
    let tok = match &ast.items[0] {
        syn::Item::Const(_) => todo!(),
        syn::Item::Enum(_) => todo!(),
        syn::Item::ExternCrate(_) => todo!(),
        syn::Item::Fn(fn_) => &fn_.block,
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
    };

    tok
}

// https://github.com/dtolnay/syn/blob/master/examples/dump-syntax/src/main.rs
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn single_num() {
        let code = include_str!("../../test_examples/src/add.rs");
        let ast = syn::parse_file(&code).unwrap();
        // dbg!(&ast);

        let tok = parse_fn(&ast);
        println!("{:#?}", tok);
    }
}
