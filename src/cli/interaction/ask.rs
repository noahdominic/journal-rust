// ask_user_for_location

use crate::core;

pub(in crate::cli) fn ask_user_for_location(
) -> Result<(String, core::Location), Box<dyn std::error::Error>> {
    let full_address: String = super::q_basic::prompt_user_for_string(
        "What is your usual location?",
        "[optional address specifiers], <location>",
    )?;

    let city = core::str_man::sanitise_spaces_html(core::str_man::split_location(&full_address)?);

    let api_response_native = core::geo::get_location_info(&city)?;

    let city_info = super::q_basic::prompt_user_for_choice(
        "There are no locations in the database with that name.",
        "There are multiple locations found.  Which one is correct?",
        "Enter the number of the correct location",
        "That doesn't seem to be one of the choices.",
        "Too many failed inputs.",
        &api_response_native,
    )?;

    println!("\nYou are currently in {city_info}.");

    Ok((full_address, city_info.clone()))
}
