mod virtualterminal;

fn main() {
    match virtualterminal::enable() {
        Ok(_) => {
            println!("\x1B[36msuccess\x1B[0m");
        }
        Err(err) => {
            println!("error: {:?}",err);
        }
    }
    println!("\x1B[36m(AFTER)\x1B[0m");
}
