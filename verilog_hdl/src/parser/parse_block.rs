use crate::generate_main::Serializer;

impl Serializer {
    pub(crate) fn parse_block(&mut self, block: &syn::Block) {
        for stmt in &block.stmts {
            match stmt {
                syn::Stmt::Local(_) => todo!(),
                syn::Stmt::Item(_) => todo!(),
                syn::Stmt::Expr(expr, _semi) => self.parse_expr(expr),
                syn::Stmt::Macro(_) => todo!(),
            }
        }
    }

    pub(crate) fn parse_expr(&mut self, expr: &syn::Expr) {
        match expr {
            syn::Expr::Array(_) => todo!(),
            syn::Expr::Assign(_) => todo!(),
            syn::Expr::Async(_) => todo!(),
            syn::Expr::Await(_) => todo!(),
            syn::Expr::Binary(expr_binary) => {
                let syn::ExprBinary {
                    attrs: _,
                    left,
                    op,
                    right,
                } = expr_binary;

                self.parse_expr(left);
                self.hdl += " ";
                self.parse_op(op);
                self.hdl += " ";
                self.parse_expr(right);
            }
            syn::Expr::Block(_) => todo!(),
            syn::Expr::Break(_) => todo!(),
            syn::Expr::Call(_) => todo!(),
            syn::Expr::Cast(_) => todo!(),
            syn::Expr::Closure(_) => todo!(),
            syn::Expr::Const(_) => todo!(),
            syn::Expr::Continue(_) => todo!(),
            syn::Expr::Field(_) => todo!(),
            syn::Expr::ForLoop(_) => todo!(),
            syn::Expr::Group(_) => todo!(),
            syn::Expr::If(_) => todo!(),
            syn::Expr::Index(_) => todo!(),
            syn::Expr::Infer(_) => todo!(),
            syn::Expr::Let(_) => todo!(),
            syn::Expr::Lit(_) => todo!(),
            syn::Expr::Loop(_) => todo!(),
            syn::Expr::Macro(_) => todo!(),
            syn::Expr::Match(_) => todo!(),
            syn::Expr::MethodCall(_) => todo!(),
            syn::Expr::Paren(_) => todo!(),
            syn::Expr::Path(path) => {
                // TODO: multi path
                // module_name::left
                // -> ["module_name", "left"]
                // -> module_name.left
                assert!(
                    path.path.segments.len() == 1,
                    "Unsupported multi path segment.(e.g. module::ident)"
                );
                self.hdl += &path.path.segments[0].ident.to_string();
            }
            syn::Expr::Range(_) => todo!(),
            syn::Expr::Reference(_) => todo!(),
            syn::Expr::Repeat(_) => todo!(),
            syn::Expr::Return(_) => todo!(),
            syn::Expr::Struct(_) => todo!(),
            syn::Expr::Try(_) => todo!(),
            syn::Expr::TryBlock(_) => todo!(),
            syn::Expr::Tuple(_) => todo!(),
            syn::Expr::Unary(_) => todo!(),
            syn::Expr::Unsafe(_) => todo!(),
            syn::Expr::Verbatim(_) => todo!(),
            syn::Expr::While(_) => todo!(),
            syn::Expr::Yield(_) => todo!(),
            _ => todo!(),
        };
    }

    pub(crate) fn parse_op(&mut self, op: &syn::BinOp) {
        self.hdl += match op {
            syn::BinOp::Add(_) => "+",
            syn::BinOp::Sub(_) => "-",
            syn::BinOp::Mul(_) => "*",
            syn::BinOp::Div(_) => "/",
            syn::BinOp::Rem(_) => "%",
            syn::BinOp::And(_) => "&",
            syn::BinOp::Or(_) => "|",
            syn::BinOp::BitXor(_) => "^",
            syn::BinOp::BitAnd(_) => "&",
            syn::BinOp::BitOr(_) => "|",
            syn::BinOp::Shl(_) => "<<",
            syn::BinOp::Shr(_) => ">>",
            syn::BinOp::Eq(_) => "==",
            syn::BinOp::Lt(_) => "<",
            syn::BinOp::Le(_) => "<=",
            syn::BinOp::Ne(_) => "!=",
            syn::BinOp::Ge(_) => ">",
            syn::BinOp::Gt(_) => ">=",
            syn::BinOp::AddAssign(_) => todo!(), // left += 1;
            syn::BinOp::SubAssign(_) => todo!(),
            syn::BinOp::MulAssign(_) => todo!(),
            syn::BinOp::DivAssign(_) => todo!(),
            syn::BinOp::RemAssign(_) => todo!(),
            syn::BinOp::BitXorAssign(_) => todo!(),
            syn::BinOp::BitAndAssign(_) => todo!(),
            syn::BinOp::BitOrAssign(_) => todo!(),
            syn::BinOp::ShlAssign(_) => todo!(),
            syn::BinOp::ShrAssign(_) => todo!(),
            _ => todo!(),
        }
    }
}
