use std::path::Path;

pub mod generate_main;
pub mod parser;
pub use generate_main::generate_code;

pub fn write_hdl(path: impl AsRef<Path>, content: &str) {
    std::fs::write(path, content).unwrap();
}

// https://github.com/dtolnay/syn/blob/master/examples/dump-syntax/src/main.rs
#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn add_num() {
        let code = r#"
pub fn add(left: usize, right: usize) -> usize {
    left + right
}"#;

        #[rustfmt::skip]
        let expected: &str =
r#"module add(input left, input right, output num);
    num = left + right;
endmodule"#;

        let ast = syn::parse_file(&code).unwrap();
        println!("{:#?}", &ast);

        let result = generate_main::generate_code(ast);
        assert_eq!(result, expected);
        write_hdl("./results/add.v", &result);
    }
}
