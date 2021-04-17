use std::collections::HashMap;

const PADDING: u8 = u8::MAX;


pub struct Chunks<I> {
    iter: I
}

impl<I> Chunks<I> {
    pub fn new(iter: I) -> Self {
        Self { iter }
    }
}

impl<I: Iterator<Item=u8>> Iterator for Chunks<I> {
    type Item = Vec<char>;

    fn next(&mut self) -> Option<Vec<char>> {
        let first = self.iter.next()?;
        let second = self.iter.next()?;
        let third = self.iter.next()?;
        let fourth = self.iter.next()?;

        Some(decode_chunk(&[first, second, third, fourth]))
    }
}


trait Chunked where Self: Sized {
    fn decode_base64_chunks(self) -> Chunks<Self>;
}

impl<A> Chunked for A where A: Iterator<Item=u8> {

    fn decode_base64_chunks(self) -> Chunks<Self> {
        Chunks::new(self)
    }
}


fn decode_first_char(first: &u8, second: &u8) -> char {
    char::from((first << 2) | ((second & 0b110000) >> 4))
}


fn decode_second_char(second: &u8, third: &u8) -> char {
    char::from(((second & 0b001111) << 4) | ((third & 0b111100) >> 2))
}


pub fn decode_chunk(chunk: &[u8; 4]) -> Vec<char> {
    match chunk {
        [a, b, PADDING, PADDING] => {
            vec![decode_first_char(a, b)]
        },
        [a, b, c, PADDING] => {
            vec![
                decode_first_char(a, b), 
                decode_second_char(b, c)
            ]
        },
        [a, b, c, d] => {
            vec![
                decode_first_char(a, b), 
                decode_second_char(b, c), 
                char::from(((c & 0b000011) << 6) | d)
            ]
        }
    }
}


// pub fn base64_to_string(map: &HashMap<char, u8>, string: &str) -> Result<Vec<u8>, String> {
//     let result =
//         string
//             .chars()
//             .map(|c| {
//                 map.get(&c).copied()
//             })
//             .decode_base64_chunks()
//             .flatten()
//             .collect::<String>();
// }

pub struct Base64 {
    table: [char; 64],
    map: HashMap<char, u8>
}


impl Base64 {

    pub fn new() -> Self {
        let mut map = HashMap::new();
        map.insert('A', 0);
        map.insert('B', 1);
        map.insert('C', 2);
        map.insert('D', 3);
        map.insert('E', 4);
        map.insert('F', 5);
        map.insert('G', 6);
        map.insert('H', 7);
        map.insert('I', 8);
        map.insert('J', 9);
        map.insert('K', 10);
        map.insert('L', 11);
        map.insert('M', 12);
        map.insert('N', 13);
        map.insert('O', 14);
        map.insert('P', 15);
        map.insert('Q', 16);
        map.insert('R', 17);
        map.insert('S', 18);
        map.insert('T', 19);
        map.insert('U', 20);
        map.insert('V', 21);
        map.insert('W', 22);
        map.insert('X', 23);
        map.insert('Y', 24);
        map.insert('Z', 25);
        map.insert('a', 26);
        map.insert('b', 27);
        map.insert('c', 28);
        map.insert('d', 29);
        map.insert('e', 30);
        map.insert('f', 31);
        map.insert('g', 32);
        map.insert('h', 33);
        map.insert('i', 34);
        map.insert('j', 35);
        map.insert('k', 36);
        map.insert('l', 37);
        map.insert('m', 38);
        map.insert('n', 39);
        map.insert('o', 40);
        map.insert('p', 41);
        map.insert('q', 42);
        map.insert('r', 43);
        map.insert('s', 44);
        map.insert('t', 45);
        map.insert('u', 46);
        map.insert('w', 47);
        map.insert('x', 48);
        map.insert('y', 49);
        map.insert('z', 50);
        map.insert('1', 51);
        map.insert('2', 52);
        map.insert('3', 53);
        map.insert('4', 54);
        map.insert('5', 55);
        map.insert('6', 56);
        map.insert('7', 57);
        map.insert('8', 58);
        map.insert('9', 59);
        map.insert('+', 61);
        map.insert('/', 62);
        map.insert('=', PADDING);

        Self {
            map,
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
    // https://en.wikipedia.org/wiki/Base64

    #[test]
    fn given_a_base64_chunk_when_there_is_no_padding_returns_vec_of_3_chars() {
        let expected = decode_chunk(&[19, 22, 5, 46]);

        assert_eq!(3, expected.len());
        assert_eq!('M', expected[0]);
        assert_eq!('a', expected[1]);
        assert_eq!('n', expected[2]);
    }


    #[test]
    fn given_a_base64_chunk_when_there_is_one_padding_returns_vec_of_2_chars() {
        let expected = decode_chunk(&[19, 22, 5, PADDING]);

        assert_eq!(2, expected.len());
        assert_eq!('M', expected[0]);
        assert_eq!('a', expected[1]);
    }

    #[test]
    fn given_a_base64_chunk_when_there_are_two_paddings_returns_vec_of_1_chars() {
        let expected = decode_chunk(&[19, 22, PADDING, PADDING]);

        assert_eq!(1, expected.len());
        assert_eq!('M', expected[0]);
    }
}