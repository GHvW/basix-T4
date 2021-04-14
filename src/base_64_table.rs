use std::collections::HashMap;

// todo
pub fn char_to_char(c: char) -> char {
    c
}

pub struct Base64 {
    table: [char; 64],
    map: HashMap<char, u8>
}

impl Base64 {

    pub fn encode_string(&self, string: &str) -> String {
        string
            .chars()
            .map(|c| char_to_char(c))
            .collect()
    }

    pub fn it(&self, string: &str) -> Vec<u8> {
        string
            .chars()
            .map(|c| self.map.get(&c).clone())
            .collect()
    }

    pub fn new() -> Self {
        let mut map = HashMap::new();
        map.insert('T', 19);
        map.insert('W', 22);
        map.insert('F', 5);
        map.insert('u', 46);

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