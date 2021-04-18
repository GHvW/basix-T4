pub mod decode;
pub mod encode;


#[cfg(test)]
mod tests {
    use super::*;
    use decode::Base64Decoder;

    #[test]
    fn a_test_of_things() {

        let it = Base64Decoder::new().decode("aGVsbG8gd29ybGQhISE=").iter().map(|it| char::from(*it)).collect::<String>();
        assert_eq!("hello world!!!", it);
    }
}
