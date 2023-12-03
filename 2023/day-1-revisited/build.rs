use std::{path::Path, env, fs};

fn main() {
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("input.rs");

    let input = fs::read_to_string("input.txt").unwrap();
    let lines = input.lines().collect::<Vec<_>>();
    fs::write(
        dest_path,
        format!(
            "static INPUT: [&'static str; {}] = [{}];",
            lines.len(),
            lines.iter()
                .map(|line| format!("\"{line}\""))
                .collect::<Vec<_>>()
                .join(",")
        )
    ).unwrap();

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=input.txt");
}