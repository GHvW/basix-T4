// need to handle padding
pub mod base_64;


#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn text_as_binary_test() {
        let b = "TWFu".as_bytes();
        for byte in b.iter() {
            println!("byte: {}", byte);
        }

        let it = "hello world".as_bytes().chunks(3);
    }
}
