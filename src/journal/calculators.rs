/**
 * Generates the path to today's entry based on the provided base directory.
 *
 * This function takes a `base_dir` as input and generates the path to today's entry
 * by concatenating the base directory with the current year, month, and day.
 * It returns a `Result` containing the generated path as a `String` on success,
 * or a boxed `dyn std::error::Error` on failure.
 *
 * # Arguments
 *
 * - `base_dir`: A `PathBuf` representing the base directory for the entries.
 *
 * # Returns
 *
 * - `Result<String, Box<dyn std::error::Error>>`: A `Result` that contains the generated path to today's entry
 *   as a `String` on success, or an error on failure.
 *
 * # Example
 *
 * ```no_run
 * use std::path::PathBuf;
 *
 * fn main() -> Result<(), Box<dyn std::error::Error>> {
 *     let base_directory = PathBuf::from("/path/to/entries");
 *     let today_entry_path: String = get_path_to_todays_entry(base_directory)?;
 *     println!("Today's entry path: {}", today_entry_path);
 *     Ok(())
 * }
 * ```
 */
pub(crate) fn get_path_to_todays_entry(base_dir: std::path::PathBuf) -> Result<String, Box<dyn std::error::Error>> {
    // Get the current date
    let current_date = chrono::Local::now().date_naive();

    // Create the path to today's entry
    let today_entry_path = format!(
        "{}/{}",
        base_dir.to_string_lossy(),
        current_date.format("%Y/%m/%d")
    );

    // Return the path as a String
    Ok(today_entry_path)
}

/// Returns the cardinal direction as a string based on the given degrees.
///
/// # Arguments
///
/// * `degrees` - The degrees to convert to a cardinal direction. Should be between 0 and 360.
///
/// # Example
///
/// ```
/// let direction = get_direction(45.0);
/// assert_eq!(direction, "NE");
/// ```
pub(crate) fn get_direction(degrees: f64) -> String {
    let directions = [
        "N", "NNE", "NE", "ENE", "E", "ESE", "SE", "SSE", "S", "SSW", "SW", "WSW", "W", "WNW",
        "NW", "NNW",
    ];
    let index = ((degrees + 11.25) / 22.5) as usize % 16;
    return String::from(directions[index]);
}
