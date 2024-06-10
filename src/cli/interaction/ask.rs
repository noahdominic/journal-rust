// ask_user_for_location

use crate as journey2;

pub(crate) fn ask_user_for_location(
) -> Result<(String, journey2::core::Location), journey2::cli::interaction::InteractionError> {
    let full_address: String = super::q_basic::prompt_user_for_string(
        "What is your usual location?",
        "[optional address specifiers], <location>",
    )?;

    let city = journey2::core::str_man::sanitise_spaces_html(
        journey2::core::str_man::split_location(&full_address),
    );

    let api_response_native = journey2::core::geo::get_location_info(&city)?;

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

pub(crate) fn ask_for_editor() -> Result<String, journey2::cli::interaction::InteractionError> {
    let choice = super::q_basic::prompt_user_for_usize(
        r#"Which text editor would you like to use?  (You have to install this separately.)
    1. Vim
    2. Emacs
    3. Nano
    4. Pico
    5. Other..."#,
    )?;

    let command = match choice {
        1 => "vim".to_string(),
        2 => "emacs".to_string(),
        3 => "nano".to_string(),
        4 => "pico".to_string(),
        5 => ask_for_custom_editor_input()?,
        _ => {
            // Early return here.
            return Err(journey2::cli::interaction::InteractionError::from(
                std::io::Error::new(
                    std::io::ErrorKind::Unsupported,
                    "Uh oh!  Something happened that shouldn't have happened.",
                ),
            ));
        }
    };

    Ok(command)
}

fn ask_for_custom_editor_input() -> std::io::Result<String> {
    let command = super::q_basic::prompt_user_for_string(
        "What program would you use to call your favourite text editor?",
        "vim | nano | pico | ...",
    )?;

    Ok(command)
}

pub(crate) fn ask_if_to_overwrite_config() -> std::io::Result<bool> {
    if !super::q_basic::prompt_user_for_bool(
        "It seems like Journey already been initialised. \
        Would you like to overwrite the current existing config file with your new details?",
    )? {
        println!("Config initialisation cancelled.");

        return Ok(false);
    }

    Ok(true)
}
