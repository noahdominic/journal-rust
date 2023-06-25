pub(crate) fn ask_for_location(
) -> Result<(String, crate::journal::Location), Box<dyn std::error::Error>> {
    let location_full_address: String = crate::journal::query::for_string(
        "What's your current location?",
        "[optional address specifiers], <location>",
    )?;

    // Getting location info via API below...
    let api_response_native =
        crate::journal::query::api::for_location_info(&location_full_address)?;

    // Let user choose which location they want
    let city_info =
        crate::journal::query::user::ask_to_choose_location_from_list(&api_response_native)?;

    println!("\nYou are currently in {}.", city_info);

    Ok((location_full_address, city_info.clone()))
}

pub(crate) fn ask_to_choose_location_from_list(
    api_results: &[crate::journal::Location],
) -> Result<&crate::journal::Location, Box<dyn std::error::Error>> {
    if api_results.is_empty() {
        Err("There are no locations in the database with that name.".into())
    } else if api_results.len() == 1 {
        Ok(&api_results[0])
    } else {
        println!(
            "There are multiple locations found with the name '{}'.  Which one is correct?",
            api_results[0].name
        );
        for (i, result) in api_results.iter().enumerate() {
            println!("{}. {}", i + 1, result);
        }
        loop {
            let choice: usize =
                crate::journal::query::for_usize("Enter the number of your correct location")?;
            if choice > 0 && choice <= api_results.len() {
                return Ok(&api_results[choice - 1]);
            } else {
                println!("That doesn't seem to be one of the choices.");
                continue;
            }
        }
    }
}
