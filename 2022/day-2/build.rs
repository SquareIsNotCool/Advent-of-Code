// Convert the input to Rust data

use std::env;
use std::fs;
use std::path::Path;

fn main() {
    println!("cargo:rerun-if-changed=input.txt");

    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("input.rs");

    let input = fs::read_to_string("input.txt").unwrap();

    let strategies: Vec<String> = input
        .trim()
        .split("\n")
        .map(
            |strategy| format!(
                "({})",
                strategy
                    .split(" ")
                    .map(|shape|
                        match shape.trim() {
                            "A" => "Shape::Rock",
                            "B" => "Shape::Paper",
                            "C" => "Shape::Scissors",
                            "X" => "Strategy::X",
                            "Y" => "Strategy::Y",
                            "Z" => "Strategy::Z",
                            _ => panic!()
                        }.to_string()
                    )
                    .collect::<Vec<String>>()
                    .join(", ")
            )
        )
        .collect();

    fs::write(
        &dest_path,
        format!(
            "const STRATEGIES: [(Shape, Strategy); {}] = [{}];",
            strategies.len(),
            strategies.join(", ")
        )
    ).unwrap();
    println!("cargo:rerun-if-changed=build.rs");
}