const MESSAGE_GREETING_CONFIG_INIT: &str = r#"


--Welcome to journal_CLI!--

This command-line interface app is here to help you document your thoughts,
experiences, and ideas effortlessly.  Let's get you started :)

For this part, we'll set your defaults.
"#;

const MESSAGE_LOCATION_EXPLAINER: &str = r#"
Let's start with your default location.  

We use your default location to automatically detect your timezome and 
the weather.  This will also be printed in your entries.  To ensure the
best results, make sure that the last part of your location is somewhere 
that is specific enough for accurate timezone and weather data.
"#;

pub(crate) fn init_new_config_driver() -> Result<(), Box<dyn std::error::Error>> {
    println!("{}", MESSAGE_GREETING_CONFIG_INIT);
    println!("{}", MESSAGE_LOCATION_EXPLAINER);

    // default_location_name and default_location are separate bc
    //      default_location_name IS user input
    //      but default_locaiton IS api information based on last substring of default_location_name
    let (default_location_name, default_location) =
        crate::journal::query::user::ask_for_location()?;

    let config_contents = format!(
        "[defaults]\n\
        location_full_name=\"{}\"\n\
        location_latitude=\"{}\"\n\
        location_longitude=\"{}\"\n\
        timezone=\"{}\"\n",
        default_location_name,
        default_location.latitude,
        default_location.longitude,
        default_location.timezone
    );

    println!(
        "\nHere are the settings we've made for you: \n{}",
        config_contents
    );

    // Ask user for path of config file
    //      Prompt: Where do you want to put config.toml?
    let config_file_path = crate::journal::query::user::ask_for_config_file_path()?;

    // If it doesn't exist, create the directories; return the PathBuf of created/existing path
    let config_file_pathbuf = crate::journal::file::mkdir_p(config_file_path)?;

    // Add filename to that PathBuf
    let config_file_pathbuf = config_file_pathbuf.join("config.toml");

    // Check for file if file already exists
    let proceed_with_writing = crate::journal::file::handle_file_exists(&config_file_pathbuf)?;

    if !proceed_with_writing {
        // Early return.  No file writing needed
        return Ok(());
    }

    // Write the settings to the path
    let mut file = std::fs::File::create(&config_file_pathbuf)?;
    std::io::Write::write_all(&mut file, config_contents.as_bytes())?;

    // Write the path to config.toml to ~/.journal
    let dotfile_pathbuf = crate::journal::file::get_dotfile_path()?;
    let mut dotfile = std::fs::File::create(&dotfile_pathbuf)?;
    std::io::Write::write_all(
        &mut dotfile,
        config_file_pathbuf.to_string_lossy().as_bytes(),
    )?;

    Ok(())
}

pub(crate) fn create_new_entry_driver() {}
