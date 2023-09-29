fn main() {
    let _ = virtualterminal::enable_stdout().unwrap();
    println!("\x1B[36m(enabled)\x1B[0m");
}
