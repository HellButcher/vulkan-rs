use std::process::Command;

fn main() {
    let status = Command::new("python3").arg("generate-sources.py").status().unwrap();
    println!("process exited with: {}", status);
}
