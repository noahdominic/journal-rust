/*
 * Copyright 2023  Noah Dominic Miranda Silvio
 *
 * Licensed under the EUPL v1.2
 */

mod journal;

// TODO:
// - Offer choice to not have default location.   This will be done by typing
//      'prefer not to say' or something like that.  Maybe a bool prompt?
//      Or maybe when they put an empty string for the location?

fn main() {
    match journal::journal_main_driver() {
        Ok(_) => (),
        Err(e) => println!("Error: {}", e),
    }
}
