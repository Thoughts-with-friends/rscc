// use super::Item;
// use syn::{punctuated::Punctuated, token::Comma};
use syn::Expr;

pub struct Block {
    pub block: Vec<String>,
}

pub fn parse_block_main(f: &syn::ItemFn) -> Block {
    Block {
        block: parse_block(&f.block),
    }
}

pub fn parse_block(block: &syn::Block) -> Vec<String> {
    let block_str = Vec::new();

    for stmt in &block.stmts {
        match stmt {
            syn::Stmt::Local(_) => todo!(),
            syn::Stmt::Item(_) => todo!(),
            syn::Stmt::Expr(expr, _) => match_expr(expr, block_str.clone()),
            syn::Stmt::Macro(_) => todo!(),
        }
    }
    block_str
}

pub fn match_expr(expr: &syn::Expr, mut block_str: Vec<String>) {
    match expr {
        syn::Expr::Array(_) => todo!(),
        syn::Expr::Assign(_) => todo!(),
        syn::Expr::Async(_) => todo!(),
        syn::Expr::Await(_) => todo!(),
        syn::Expr::Binary(expr_binary) => {
            let left = match_box_expr(expr_binary.left.clone());
            let right = match_box_expr(expr_binary.right.clone());
            block_str.push(left);
            block_str.push(right);
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
          for segment in  &path.path.segments{
            block_str.push(segment.ident.to_string());
          }
        },
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
    }
}

pub fn match_box_expr(expr: Box<syn::Expr>) -> String {
    match *expr {
        Expr::Array(_) => todo!(),
        Expr::Assign(_) => todo!(),
        Expr::Async(_) => todo!(),
        Expr::Await(_) => todo!(),
        Expr::Binary(_) => todo!(),
        Expr::Block(_) => todo!(),
        Expr::Break(_) => todo!(),
        Expr::Call(_) => todo!(),
        Expr::Cast(_) => todo!(),
        Expr::Closure(_) => todo!(),
        Expr::Const(_) => todo!(),
        Expr::Continue(_) => todo!(),
        Expr::Field(_) => todo!(),
        Expr::ForLoop(_) => todo!(),
        Expr::Group(_) => todo!(),
        Expr::If(_) => todo!(),
        Expr::Index(_) => todo!(),
        Expr::Infer(_) => todo!(),
        Expr::Let(_) => todo!(),
        Expr::Lit(_) => todo!(),
        Expr::Loop(_) => todo!(),
        Expr::Macro(_) => todo!(),
        Expr::Match(_) => todo!(),
        Expr::MethodCall(_) => todo!(),
        Expr::Paren(_) => todo!(),
        Expr::Path(expr_path) => {
            // crate::right -> vec!["crate", "right"]
            let mut paths = String::new();
            for path_seg in expr_path.path.segments {
                paths = path_seg.ident.to_string();
            }
            paths
        }
        Expr::Range(_) => todo!(),
        Expr::Reference(_) => todo!(),
        Expr::Repeat(_) => todo!(),
        Expr::Return(_) => todo!(),
        Expr::Struct(_) => todo!(),
        Expr::Try(_) => todo!(),
        Expr::TryBlock(_) => todo!(),
        Expr::Tuple(_) => todo!(),
        Expr::Unary(_) => todo!(),
        Expr::Unsafe(_) => todo!(),
        Expr::Verbatim(_) => todo!(),
        Expr::While(_) => todo!(),
        Expr::Yield(_) => todo!(),
        _ => todo!(),
    }
}
