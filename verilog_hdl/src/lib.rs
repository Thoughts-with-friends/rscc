use std::path::Path;

pub mod generate_main;
pub mod parser;

pub fn write_hdl(path: impl AsRef<Path>, content: &str) {
    std::fs::write(path, content).unwrap();
}

// https://github.com/dtolnay/syn/blob/master/examples/dump-syntax/src/main.rs
#[cfg(test)]
mod tests {
    use crate::generate_main::Serializer;

    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn output_ast() {
        let code = r#"
        pub fn add(left: usize, right: usize) -> usize {
            let num = left + right;
            num
        }"#;

        let rust_ast = syn::parse_file(code).unwrap();
        println!("{rust_ast:#?}");
    }

    #[test]
    fn add_num() {
        let code = r#"
pub fn add(left: usize, right: usize) -> usize {
    left + right
}"#;

        let ser = Serializer::default();
        let result = ser.generate_code(code);

        #[rustfmt::skip]
        let expected: &str =
r#"module add(input left, input right, output num);
    num = left + right;
endmodule"#;

        assert_eq!(result, expected);
        write_hdl("./results/add.v", &result);
    }
}
