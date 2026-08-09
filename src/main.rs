fn main() {
    let loud = std::env::var("HELLO_LOUD").is_ok();
    let msg = "Hello, world!";
    if loud {
        println!("{}", msg.to_uppercase());
    } else {
        println!("{}", msg);
    }
    let n: usize = std::env::var("HELLO_REPEAT")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(1);
    for _ in 0..n {
        println!("Hello, world!");
    }
}
