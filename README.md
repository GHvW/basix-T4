# basix-T4

A basic Base64 encoder and decoder. Not focused on performance. Focused on working. Over and under engineering are not concerns and are both likely :laughing:

Sample decode
```rust
use basixT4::decode::Base64Decoder;

fn main() {
    let hello_world = 
        Base64Decoder::new()
            .decode("aGVsbG8gd29ybGQhISE=")
            .iter()
            .map(|it| char::from(*it))
            .collect::<String>();

    println!("{}", hello_world);
}
```


Sample encode
```rust
use basixT4::encode::Base64Encoder;

fn main() {
    let hello_world = 
        Base64Encoder::new()
            .encode("hello world!!!".as_bytes());

    println!("{}", hello_world);
}
```