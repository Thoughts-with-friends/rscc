pub mod parse_expr;
pub mod parse_local;

use crate::generate_main::Serializer;

impl Serializer {
    pub(crate) fn parse_block(&mut self, block: &syn::Block) {
        for stmt in &block.stmts {
            match stmt {
                syn::Stmt::Local(local) => self.parse_local(local),
                syn::Stmt::Item(_) => todo!(),
                syn::Stmt::Expr(expr, _semi) => self.parse_expr(expr),
                syn::Stmt::Macro(_) => todo!(),
            }
        }
    }
}
