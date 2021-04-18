use basixT4::encode::Base64Encoder;

fn main() {
    let hello_world = 
        Base64Encoder::new()
            .encode("hello world!!!".as_bytes());

    println!("{}", hello_world);
}