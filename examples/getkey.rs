fn main() {
    if let Ok(_) = virtualterminal::enable_stdin() {
        if let Ok(_) = virtualterminal::make_raw() {
            if let Ok(key) = virtualterminal::getkey() {
                println!("{}", key.replace("\x1B","ESC") );
            }
        }
    }
}
