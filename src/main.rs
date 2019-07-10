use std::fs;
mod tokenizer;
fn main() {
    let filename = "examples/hello_world.slam";
    let contents = fs::read_to_string(filename).expect("something went wrong reading the file");
    let tokens = tokenizer::tokenize(contents);
    for token in tokens {
        println!("{:?}", token);
    }
}
