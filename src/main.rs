mod journal;

fn main() {
    match journal::main_driver() {
        Ok(_) => (),
        Err(e) => println!("Error: {}", e),
    }
}