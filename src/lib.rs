// need to handle padding
pub mod decode;
pub mod encode;

use crate::decode::Base64Decoder;

#[cfg(test)]
mod tests {
    use super::*;
    use decode::Base64Decoder;

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

    #[test]
    fn a_test_of_things() {

        let it = Base64Decoder::new().decode("aGVsbG8gd29ybGQhISE=").iter().map(|it| char::from(*it)).collect::<String>();
        assert_eq!("hello world!!!", it);
    }
}
