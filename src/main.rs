mod var_examples;
use var_examples::variables;
use var_examples::tell_height;
use var_examples::human_info;

fn main() {
    hello_world();
    variables();
    tell_height(179);
    human_info("Alice", 30, 180.5);
}

fn hello_world() {
    println!("Hello, world!");
}
