use encoding::hex;

pub fn hello() {
    let hello = b"hello";
    println!("{}", hex::encode_to_string(hello));
}
