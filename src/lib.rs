// letters are 6 bits, correspond to letters in the table
// Ex 'T' is 010011 which = 19 in decimal. In the base64 table, 19 = T
// consider returning 3 byte chunks as an iterator and then flattening them
// need to handle padding
pub mod base_64_table;

type Chunk = [u8; 3];

// get actual 
// pub fn convert_chunk(chunk: &Chunk) -> Vec<char> {

//     let first = (chunk[0] & 0b11111100) >> 2;
//     let second = ((chunk[0] & 0b00000011) << 4) | ((chunk[1] & 0b11110000) >> 4);
//     let third = ((chunk[1] & 0b00001111) << 2) | (chunk[2])
// }


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
}
