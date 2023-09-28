mod virtualterminal;

fn main() {
    if let Ok(_) = virtualterminal::enable_stdout() {
        println!("\x1B[36msuccess\x1B[0m");
    }
    println!("\x1B[36m(AFTER)\x1B[0m");
}
