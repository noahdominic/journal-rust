/*******************************************************************************
 * Copyright (c) 2023, 2024  Noah Dominic Miranda Silvio.  All rights reserved
 * Licensed under the EUPL v1.2
 ******************************************************************************/

/// Returns the cardinal direction as a string based on the given degrees.
///
/// # Arguments
///
/// * `degrees` - The degrees to convert to a cardinal direction. Should be between 0 and 360.
pub(crate) fn get_direction(degrees: f64) -> String {
    let directions = [
        "N", "NNE", "NE", "ENE", "E", "ESE", "SE", "SSE", "S", "SSW", "SW", "WSW", "W", "WNW",
        "NW", "NNW",
    ];
    let index = ((degrees + 11.25) / 22.5) as usize % 16;
    return String::from(directions[index]);
}
