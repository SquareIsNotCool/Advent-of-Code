// Convert the input to an array of arrays at build time for simplicity

use std::env;
use std::fs;
use std::path::Path;

fn main() {
    println!("cargo:rerun-if-changed=input.txt");

    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("input.rs");

    let input = fs::read_to_string("input.txt").unwrap();

    let mut elves: Vec<Vec<&str>> = input
        .trim()
        .split("\n\r\n")
        .map(|elf| elf.split("\n").map(|calories| calories.trim()).collect::<Vec<&str>>())
        .collect();

    let most_food_carried_by_an_elf = elves.iter().map(|elf| elf.len()).fold(0, usize::max);

    for elf in elves.iter_mut() {
        elf.resize(most_food_carried_by_an_elf, "0");
    };

    fs::write(
        &dest_path,
        format!(
            "\
            const ELVES: [[u32; {}]; {}] = [{}];\
            ",
            most_food_carried_by_an_elf,
            elves.len(),
            elves.iter().map(|elf| format!("[{}]", elf.join(", "))).collect::<Vec<String>>().join(", ")
        )
    ).unwrap();
    println!("cargo:rerun-if-changed=build.rs");
}