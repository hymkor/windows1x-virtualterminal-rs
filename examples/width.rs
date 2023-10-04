fn main() {
    let w = virtualterminal::width_stdout().unwrap();
    println!("width={}", w);
}
