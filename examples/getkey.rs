fn main() {
    if let Ok(_) = virtualterminal::enable_stdin() {
        if let Ok(_) = virtualterminal::make_raw() {
            if let Ok(key) = virtualterminal::getkey() {
                println!("{}", key.replace("\x1B", "ESC"));
            }
        }
        // When the return value of make_raw() is droppped,
        // the mode is changed to cooked mode.
    }
    // When the return value of enable_stdin() is droppped,
    // virtual_terminal_input is disabled.
}
