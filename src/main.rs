fn main() {
    let loud = std::env::var("HELLO_LOUD").is_ok();
    let msg = "Hello, world!";
    if loud {
        println!("{}", msg.to_uppercase());
    } else {
        println!("{}", msg);
    }
}
