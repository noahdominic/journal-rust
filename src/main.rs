// Copyright 2023  Noah Dominic Miranda Silvio
// Licensed under the EUPL v1.2

extern crate journey2;

fn main() {
    match journey2::core::handle_main() {
        Ok(_) => (),
        Err(e) => println!("Error: {}", e),
    }
}
