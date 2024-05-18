// use common::add;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

// https://github.com/dtolnay/syn/blob/master/examples/dump-syntax/src/main.rs
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn single_num() {
        let string =include_str!("../../common/rs_files/single_num.rs");
        let ast =syn::parse_file(&string).unwrap();

        println!("{:#?}", ast);
    }
}
