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
    #[cfg(feature = "write_test")]
    use crate::write_hdl;
    use pretty_assertions::assert_eq;
    use std::path::PathBuf;

    #[test]
    fn output_ast() {
        let code = r#"
        pub fn add(left: usize, right: usize) -> usize {
            let num = left + right;
            num
        }"#;

        let rust_ast = syn::parse_file(code).unwrap();

        let repo_root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("..");
        let output_file = repo_root.join("ast.txt");
        std::fs::write(output_file, format!("{rust_ast:#?}")).unwrap();
    }

    fn bin_op_test(name: &str, op: &str) -> String {
        let code = format!(
            r#"
pub fn {name}(left: usize, right: usize) -> usize {{
    left {op} right
}}"#
        );

        let ser = Serializer::default();
        let result = ser.generate_code(&code);

        #[rustfmt::skip]
        let expected = format!(
r#"module {name}(input left, input right, output num);
    num = left {op} right;
endmodule"#);

        assert_eq!(result, expected);
        result
    }

    #[test]
    fn operate_num() {
        let ops = [
            ("add", "+"),
            ("sub", "-"),
            ("mul", "*"),
            ("div", "/"),
            ("rem", "%"),
        ];
        for (name, op) in ops {
            let _result = bin_op_test(name, op);
            #[cfg(feature = "write_test")]
            {
                write_hdl(format!("./results/num_{name}.v"), &_result);
            }
        }
    }
}
