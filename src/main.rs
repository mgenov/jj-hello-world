fn main() {
    print_hello();
    print_world();
    greet("world!");
}

/// print_hello() prints hello world
fn print_hello() {
    println!("Hello, world!");
}

fn print_world() {
    println!("World! A");
}

fn welcome(name: &str) {
    println!("Hello, {}!", name);
}
