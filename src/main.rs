fn main() {
    print_hello();
    print_world();
    welcome("world!");
}

/// print_hello() prints hello world
fn print_hello() {
    println!("Hello, world!");
}

fn print_world() {
    println!("World! A");
}

/// welcome welcomes name.
fn welcome(name: &str) {
    println!("Hello, {}!", name);
}
