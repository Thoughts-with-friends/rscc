use self::parse_fn::Function;

pub mod parse_block;
pub mod parse_fn;

pub enum Item {
    Fn(Function),
}