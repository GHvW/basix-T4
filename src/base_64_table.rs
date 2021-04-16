use std::collections::HashMap;


// todo
pub fn char_to_char(c: char) -> char {
    c
}

pub struct Chunks<I> {
    iter: I
}

impl<I> Chunks<I> {
    pub fn new(iter: I) -> Self {
        Self { iter }
    }
}

impl<I: Iterator<Item=u8>> Iterator for Chunks<I> {
    type Item = Vec<u8>;

    fn next(&mut self) -> Option<Vec<u8>> {
        self.iter.next().map(|first| {
            let mut chunk = vec![first];

            self.iter.next().map(|second| {
                chunk.push(second);
                ()
            });

            self.iter.next().map(|third| {
                chunk.push(third);
                ()
            });

            chunk
        })
    }
}


trait Chunked where Self: Sized {

    fn chunked(self) -> Chunks<Self>;
}

impl<A> Chunked for A where A: Iterator<Item=u8> {

    fn chunked(self) -> Chunks<Self> {
        Chunks::new(self)
    }
}


pub fn base64_to_indexes(map: &HashMap<char, u8>, string: &str) -> Result<Vec<u8>, String> {
    let result =
        string
            .chars()
            .map(|c| {
                map.get(&c)
                .copied()
                .unwrap_or(u8::MAX)
            })
            .take_while(|it| *it != u8::MAX)
            .collect::<Vec<u8>>();

    if result.len() == string.len() {
        Ok(result)
    } else {
        Err("Provided string is invalid Base64".to_string())
    }
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