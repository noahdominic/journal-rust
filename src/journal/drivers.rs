const MESSAGE_GREETING_CONFIG_INIT: &str = "--Welcome to journal_CLI!--\n\n\
This command-line interface app is here to help you document your thoughts,\n\
experiences, and ideas effortlessly.  Let's get you started :) \n\n\
\
For this part, we'll set your defaults.";

const MESSAGE_LOCATION_EXPLAINER: &str = "Let's start with your default location.  \
We use your default location to automatically detect your timezome and the weather.  \
This will also be printed in your entries.  To ensure best results, make sure \
that the last part of your location is somewhere that is specific enough \
for accurate timezone and weather data.";

pub(crate) fn init_new_config_driver() -> Result<(), Box<dyn std::error::Error>> {
    println!("{}", MESSAGE_GREETING_CONFIG_INIT);
    println!();
    println!("{}", MESSAGE_LOCATION_EXPLAINER);

    // default_location_name and default_location are separate bc
    // default_location_name IS user input
    // but default_locaiton IS api information based on last substring of default_location_name
    let (default_location_name, default_location) =
        crate::journal::query::user::ask_for_location()?;

    println!();
    println!("Here are the settings we've made for you:");
    println!("[defaults]");
    println!("location_full_name=\"{}\"", default_location_name);
    println!("location_latitude=\"{}\"", default_location.latitude);
    println!("location_longitude=\"{}\"", default_location.longitude);
    println!("timezone=\"{}\"", default_location.timezone);

    let config_file_path = crate::journal::query::user::ask_for_config_file_path()?;
    crate::journal::file::mkdir_p(config_file_path)?;

    Ok(())
}

pub(crate) fn create_new_entry_driver() {}
