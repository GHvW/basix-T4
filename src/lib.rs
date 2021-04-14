// letters are 6 bits, correspond to letters in the table
// Ex 'T' is 010011 which = 19 in decimal. In the base64 table, 19 = T
pub mod base_64_table;

pub fn convert_chunk(chunk: &str) -> Vec<char> {
    chunk.iter()
}

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
    }

    #[test]
    fn play() {
        let it = convert_chunk("TWF")
    }
}
