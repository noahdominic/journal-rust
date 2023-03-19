mod journal;

fn main() {
    match journal::journal_main_driver() {
        Ok(_) => (),
        Err(e) => println!("Error: {}", e),
    }
}