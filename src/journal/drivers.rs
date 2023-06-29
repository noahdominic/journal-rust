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
    // default_location_name IS user input
    // but default_locaiton IS api information based on last substring of default_location_name
    let (default_location_name, default_location) =
        crate::journal::query::user::ask_for_location()?;

    let config_contents = format!(
        "[defaults]\nlocation_full_name=\"{}\"\nlocation_latitude=\"{}\"\nlocation_longitude=\"{}\"\ntimezone=\"{}\"\n",
        default_location_name,
        default_location.latitude,
        default_location.longitude,
        default_location.timezone
    );

    println!();
    println!("Here are the settings we've made for you: \n{}", config_defaults);

    let config_file_path = crate::journal::query::user::ask_for_config_file_path()?;
    let config_file_pathbuf = crate::journal::file::mkdir_p(config_file_path)?;

    let config_file_path = config_file_pathbuf.join("config.toml");

    if std::path::Path::new(&config_file_path).exists() {
        // Handle the case when the file already exists
        // For example, you might choose to prompt the user for confirmation or take a different action
        if !crate::journal::query::for_bool(&format!(
            "A config.toml already exists in {}.  Overwrite?",
            config_file_path.to_string_lossy()
        ))? {
            println!("Config init cancelled.");
            return Ok(());
        }
    }

    let mut file = std::fs::File::create(&config_file_path)?;
    std::io::Write::write_all(&mut file, config_contents.as_bytes())?;

    Ok(())
}

pub(crate) fn create_new_entry_driver() {}
