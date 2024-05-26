#[cfg(test)]
mod test1;
// #[cfg(test)]
// mod test2;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn main() {
        test1::test();
        // test2::test();
    }
}
