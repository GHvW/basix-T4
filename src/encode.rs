
fn encode_first(first: u8) -> usize {
    usize::from((first & 0b11111100) >> 2)
}

fn encode_second(first: u8, second: u8) -> usize {
    usize::from(((first & 0b00000011) << 4) | ((second & 0b11110000) >> 4))
}

fn encode_third(second: u8, third: u8) -> usize {
    usize::from(((second & 0b00001111) << 2) | ((third & 0b11000000) >> 6))
}


fn encode_one_byte(byte: u8) -> Vec<usize> {
    vec![
        encode_first(byte), 
        encode_second(byte, 0), 
    ]
}

fn encode_two_bytes(a: u8, b: u8) -> Vec<usize> {
    vec![
        encode_first(a), 
        encode_second(a, b), 
        encode_third(b, 0), 
    ]
}

fn encode_three_bytes(a: u8, b: u8, c: u8) -> Vec<usize> {
    vec![
        encode_first(a), 
        encode_second(a, b), 
        encode_third(b, c), 
        usize::from(c & 0b00111111)
    ]
}


fn map_to_base64(table: &[char; 64], chunk: Vec<usize>) -> Vec<char> {
    chunk
        .iter()
        .map(|it| table[*it])
        .collect()
}

pub struct Base64Encoder {
    table: [char; 64],
}


impl Base64Encoder {
    pub fn encode(&self, bytes: &[u8]) -> String {
        bytes
            .chunks(3)
            .map(|chunk| {
                let len = chunk.len();
                if len == 3 {

                    map_to_base64(
                        &self.table, 
                        encode_three_bytes(chunk[0], chunk[1], chunk[2]))
                } else if len == 2 {

                    let mut vec: Vec<char> = 
                        map_to_base64(
                            &self.table, 
                            encode_two_bytes(chunk[0], chunk[1]));

                    vec.push('=');
                    vec
                } else {

                    let mut vec: Vec<char> = 
                        map_to_base64(
                            &self.table, 
                            encode_one_byte(chunk[0]));

                    vec.push('=');
                    vec.push('=');
                    vec
                }
            })
            .flatten()
            .collect::<String>()
    }

    pub fn new() -> Self {
        Self {
            table: [
                'A', // 0
                'B',
                'C', 
                'D',
                'E',
                'F',
                'G',
                'H',
                'I',
                'J',
                'K', // 10
                'L',
                'M',
                'N',
                '0',
                'P',
                'Q',
                'R',
                'S',
                'T',
                'U', // 20
                'V',
                'W',
                'X',
                'Y',
                'Z',
                'a',
                'b',
                'c',
                'd',
                'e', // 30
                'f',
                'g',
                'h',
                'i',
                'j',
                'k',
                'l',
                'm',
                'n',
                'o', // 40
                'p',
                'q',
                'r',
                's',
                't',
                'u',
                'v',
                'w',
                'x',
                'y', // 50
                'z',
                '0',
                '1',
                '2',
                '3',
                '4',
                '5',
                '6',
                '7',
                '8', // 60
                '9',
                '+',
                '/' // 63
            ]
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn given_3_bytes_converts_to_6_bit_indexes_with_no_padding() {
        let expected = encode_three_bytes(77, 97, 110);

        assert_eq!(expected, [19, 22, 5, 46]);
    }


    #[test]
    fn given_2_bytes_converts_to_6_bit_indexes_with_one_padding() {
        let expected = encode_two_bytes(77, 97);

        assert_eq!(expected, [19, 22, 4]);
    }


    #[test]
    fn given_1_byte_converts_to_6_bit_indexes_with_two_paddings() {
        let expected = encode_one_byte(77);

        assert_eq!(expected, [19, 16]);
    }

    #[test]
    fn the_test() {
        let expected = Base64Encoder::new().encode("Man".as_bytes());

        assert_eq!("TWFu", expected);
    }
}