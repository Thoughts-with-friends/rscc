use crate::generate_main::Serializer;

impl Serializer {
    /// # Expected rust code
    ///
    /// ```edition2021
    /// let num = 1;
    /// ```
    ///
    /// # HDL
    /// ```SystemVerilog
    /// // int
    /// int num = 1;
    ///
    /// // wire
    /// wire signed [31:0]　left, right, num;
    /// assign num = a + b;
    ///
    /// // register
    // reg [31:0] num;
    ///
    /// // output
    /// num = 1;
    /// ```
    ///
    pub(crate) fn parse_local(&mut self, local: &syn::Local) {
        let syn::Local {
            attrs: _,
            let_token: _,
            pat, // e.g. num
            init, // = 1
            semi_token,
        } = local;

        // Pat::Ident
        match pat {
            syn::Pat::Const(_) => todo!(),
            syn::Pat::Ident(pat_ident) => self.hdl += &pat_ident.ident.to_string(),
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
        }

        if let Some(init) = init {
            // - let x: u64 = s.parse()?
            // - let Ok(x) = r else { return }
            let syn::LocalInit {
                eq_token: _,
                expr,
                diverge: _,
            } = init;

            self.parse_expr(expr);
        };
    }
}
