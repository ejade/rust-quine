use std::fs;

fn main() {
    let contents = fs::read_to_string("src/main.rs")
        .expect("Should have been able to read the file");

    println!("{contents}");
}
