use std::path::PathBuf;

mod parser;
mod vehicule;

fn main() {
    println!("Hello, world!");
    let data = parser::load(PathBuf::from("data/data101.vrp")).unwrap();
    std::println!("{data:?}");
}
