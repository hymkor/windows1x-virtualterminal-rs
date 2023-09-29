fn main() {
    if let Ok(_) = virtualterminal::enable_stdout() {
        println!("\x1B[36m(enabled)\x1B[0m");
    }
    // When the return value of enable_stdout is dropped,
    // virtual terminal processing is disabled.
    println!("\x1B[36m(disabled)\x1B[0m");
}
