use crate::parser::{
    parse_block::{parse_block_main, Block},
    parse_fn::{parse_items, Function},
    Item,
};

pub fn generate_code(rust_ast: syn::File) -> String {
    let mut hdl = String::new();

    let item = &parse_items(rust_ast.items)[0];

    match item {
        Item::Fn(f) => {
            let Function { fn_name, args } = f;

            let mut args_str = String::new();
            for v in args {
                args_str.push_str(&format!("input {}, ", v));
            }

            // let b = parse_block_main(Item::ItemFn);

            // main generator
            #[rustfmt::skip]
            hdl.push_str(&format!(
r#"module {fn_name}({args_str}output num);
    num = left + right;
endmodule"#,
            ));
        }
    }

    hdl
}
