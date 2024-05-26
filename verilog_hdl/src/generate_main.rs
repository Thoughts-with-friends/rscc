#[derive(Debug, derive_new::new)]
pub struct Serializer {
    pub hdl: String,
    pub indent: &'static str,
    depth: usize,
}

impl Default for Serializer {
    fn default() -> Self {
        Self {
            hdl: String::new(),
            indent: "    ",
            depth: 0,
        }
    }
}

impl Serializer {
    pub(crate) fn increment_depth(&mut self) {
        self.depth += 1;
    }

    pub(crate) fn decrement_depth(&mut self) {
        self.depth -= 1;
    }

    pub(crate) fn indent(&mut self) {
        match self.depth {
            0 => (),
            1 => self.hdl += &self.indent,
            _ => self.hdl += &self.indent.repeat(self.depth),
        }
    }

    pub fn generate_code(mut self, rust_code: &str) -> String {
        let rust_ast = syn::parse_file(rust_code).unwrap();

        let syn::File {
            shebang: _,
            attrs: _,
            items,
        } = rust_ast;

        // self.parse_shebang(shebang);
        // self.parse_attrs(attrs);
        self.parse_items(items);
        self.hdl
    }
}
