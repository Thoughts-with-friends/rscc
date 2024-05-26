use crate::generate_main::Serializer;

use syn::{punctuated::Punctuated, token::Comma};

impl Serializer {
    /// # Expected Verilog-HDL code
    ///
    /// ```SystemVerilog
    /// module add(input left, input right, output num);
    ///     num = left + right;  // statements
    /// endmodule
    /// ```
    pub(crate) fn parse_fn(&mut self, f: syn::ItemFn) {
        let syn::Signature {
            ident, inputs: _, ..
        } = &f.sig;

        self.hdl += "module ";
        self.hdl += &ident.to_string();
        self.parse_args(f.sig.inputs);
        self.increment_depth();
        self.indent();
        self.hdl += "num = ";
        self.parse_block(&f.block);
        self.hdl += ";\n";
        self.decrement_depth();
        self.hdl += "endmodule";
    }

    /// # Expected HDL
    /// ```SystemVerilog
    /// (input left, input right, output num)
    /// ```
    pub(crate) fn parse_args(&mut self, inputs: Punctuated<syn::FnArg, Comma>) {
        self.hdl += "(";

        for panc in inputs {
            match panc {
                syn::FnArg::Receiver(_) => todo!(),
                syn::FnArg::Typed(pad_typed) => match *pad_typed.pat {
                    syn::Pat::Const(_) => todo!(),
                    syn::Pat::Ident(pat_ident) => {
                        self.hdl += "input ";
                        self.hdl += &pat_ident.ident.to_string();
                        self.hdl += ", ";
                    }
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
        self.hdl += "output num";
        self.hdl += ");\n";
    }
}
