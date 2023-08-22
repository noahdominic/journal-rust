/*
 * Copyright 2023 Noah Dominic Miranda Silvio
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the European Union Public License version 1.2,
 * as published by the European Commission.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * European Union Public Licence for more details.
 *
 * You should have received a copy of the European Union Public Licence
 * along with this program. If not, see <https://joinup.ec.europa.eu/collection/eupl/eupl-text-eupl-12>.
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
