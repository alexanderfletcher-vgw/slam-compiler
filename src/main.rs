use std::fs;
mod tokenizer;
fn main() {
    println!("Hello, World!");
    let filename = "examples/hello_world.slam";
    let contents = fs::read_to_string(filename).expect("something went wrong reading the file");
    tokenizer::tokenize(contents);
}
