fn main() {
    if let Ok(_) = virtualterminal::enable_stdout() {
        println!("\x1B[36m(enabled)\x1B[0m");
    }
    // The return value of enable_stdout is dropped,
    // The virtual terminal processing is disabled.
    println!("\x1B[36m(disabled)\x1B[0m");
}
