/*
 * Copyright 2023  Noah Dominic Miranda Silvio
 *
 * Licensed under the EUPL v1.2
 */

pub(crate) mod open_meteo;
pub(crate) mod user;

use curl::easy::Easy;
use std::io::Write;

/// Prints a string with blank line paddings.
macro_rules! lnprint {
    ($($arg:tt)*) => {{
        println!();
        print!($($arg)*);
    }}
}

/// Prompts the user with a question and hint, reads the user's input from the standard input stream, and returns it as a string.
///
/// # Arguments
///
/// * `question` - The question to prompt the user.
/// * `hint` - A hint or additional information for the user.
///
/// # Returns
///
/// Returns the user's input as a string or an `std::io::Error` if an I/O error occurs.
pub(crate) fn for_string(question: &str, hint: &str) -> Result<String, std::io::Error> {
    lnprint!("{} ({}): ", question, hint);
    std::io::stdout().flush()?;

    let mut user_response = String::new();
    std::io::stdin().read_line(&mut user_response)?;

    Ok(user_response.trim_end().to_string())
}

/// Prompts the user with a yes/no question, reads the user's input from the standard input stream, and returns `true` if the answer is "yes" or "y", and `false` otherwise.
///
/// # Arguments
///
/// * `question` - The yes/no question to prompt the user.
///
/// # Returns
///
/// Returns `true` if the user's answer is "yes" or "y", and `false` otherwise. Returns an `std::io::Error` if an I/O error occurs.
pub(crate) fn for_bool(question: &str) -> Result<bool, std::io::Error> {
    let answer = for_string(question, "y/N")?.trim().to_lowercase();
    Ok(answer == "yes" || answer == "y")
}

/// Prompts the user with a question, reads the user's input from the standard input stream, and returns the parsed input as a `usize`.
///
/// # Arguments
///
/// * `question` - The question to prompt the user.
///
/// # Returns
///
/// Returns the parsed user input as a `usize` or a boxed error (`Box<dyn std::error::Error>`) if parsing or an `std::io::Error` occurs.
pub(crate) fn for_usize(question: &str) -> Result<usize, Box<dyn std::error::Error>> {
    let user_response = for_string(question, "[index]")?;
    Ok(user_response.trim().parse::<usize>()?)
}

/// Performs an API call to the specified URL and returns the response as a vector of bytes.
///
/// # Arguments
///
/// * `url` - The URL to make the API call to.
///
/// # Returns
///
/// Returns the API response as a vector of bytes (`Vec<u8>`) or a `curl::Error` if an error occurs during the API call.
pub(crate) fn call_api(url: &str) -> Result<Vec<u8>, curl::Error> {
    let mut api_caller = Easy::new();
    api_caller.url(url)?;
    let mut api_response_bytes = Vec::new();
    {
        let mut transfer = api_caller.transfer();
        transfer.write_function(|received_data| {
            api_response_bytes.extend_from_slice(received_data);
            Ok(received_data.len())
        })?;
        transfer.perform()?;
    }
    Ok(api_response_bytes)
}
