pub(crate) mod api;
pub(crate) mod user;

use curl::easy::Easy;
use std::io::Write;

/// Prints a string in with blank line paddings
macro_rules! lnprint {
    ($($arg:tt)*) => {{
        println!();
        print!($($arg)*);
    }}
}

/// Handles a generic query by prompting the user with a question and hint, and
/// reading their input from the standard input stream, which is returned as a `String`
///
/// # Arguments
///
/// * `question`: A string slice representing the question to be displayed to the user.
/// * `hint`: A string slice representing a hint or example value to be displayed to the user.
///
/// # Returns
///
/// An `Result<String, std::io::Error>` containing the user's input,
/// or `std::io::Error` if an error occurred while reading the input.
pub(crate) fn for_string(question: &str, hint: &str) -> Result<String, std::io::Error> {
    // Prompt the user with the question and hint.
    lnprint!("{} ({}): ", question, hint);
    std::io::stdout().flush()?;

    // Read the user's input from the standard input stream.
    let mut user_response = String::new();
    std::io::stdin().read_line(&mut user_response)?;

    // Return the user's input, with the trailing newline trimmed, as an `Ok`.
    Ok(user_response.trim_end().to_string())
}

/// Handles a generic query by prompting the user with a question and hint, and
/// reading their input from the standard input stream, which is returned as a `String`
///
/// # Arguments
///
/// * `question`: A string slice representing the question to be displayed to the user.
/// * `hint`: A string slice representing a hint or example value to be displayed to the user.
///
/// # Returns
///
/// An `Result<String, std::io::Error>` containing the user's input,
/// or `std::io::Error` if an error occurred while reading the input.
///
/// # Note
///
/// This function is dependent on `query_for_string()`.
pub(crate) fn for_bool(question: &str) -> Result<bool, std::io::Error> {
    // This is very condensed so here's a more readable summary:
    // - `query_for_string()` is called and passes `question` and `hint`.
    // - The returned string is then checked if it's "yes" or "y".
    // - This yes-ness is wrapped in an `Ok()` and returned
    let answer = for_string(question, "y/N")?.trim().to_lowercase();
    Ok(answer == "yes" || answer == "y")
}

/// Handles a generic query by prompting the user with a question and hint, and
/// reading their input from the standard input stream, which is returned as a `usize`
///
/// # Arguments
///
/// * `question`: A string slice representing the question to be displayed to the user.
/// * `hint`: A string slice representing a hint or example value to be displayed to the user.
///
/// # Returns
///
/// An `Result<usize, std::num::ParseIntError>` containing the parsed input as a `usize`,
/// or `std::num::ParseIntError` if the input could not be parsed as a `usize`.
///
/// # Note
///
/// This function is dependent on `query_for_string()`.
pub(crate) fn for_usize(question: &str) -> Result<usize, Box<dyn std::error::Error>> {
    let user_response = for_string(question, "[index]")?;
    Ok(user_response.trim().parse::<usize>()?)
}

/// Makes a generic API call to the given `url`, using the curl crate's Easy API.
/// Returns a `Result` containing the response body as a vector of bytes on success,
/// or an error wrapped in a `Box<dyn std::error::Error>` on failure.
///
/// # Arguments
///
/// * `url` - A string slice representing the URL to call the API on.
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
