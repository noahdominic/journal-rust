use crate::journal::file::FileError;

const MESSAGE_GREETING_CONFIG_INIT: &str = r#"

--Welcome to journal_CLI!--

This command-line interface app is here to help you document your thoughts,
experiences, and ideas effortlessly.  Let's get you started :)

For this part, we'll set your defaults.
"#;

const MESSAGE_LOCATION_EXPLAINER: &str = r#"
We'll only need your usual location.  

We use your default location to automatically detect your default timezome and 
to detect the current weather.  This will also be printed in your entries.  
To ensure the best results, make sure that the last part of your location is 
somewhere that is specific enough for accurate timezone and weather data.

Don't worry---if your city has the same name as a city elsewhere,
like Los Angeles, Los Santos or San Francisco, Cebu,
you would be asked to pick which city you meant.

Example:
- Avenida 9 SO - Carchi, Guiyaquil
- Lor Marzuki, Singapore City
- Café What?, Moshoeshoe Rd, Maseru
"#;

const MESSAGE_EDITORS_EXPLAINER: &str = r#"
This application does not use its own text editors and will separately run 
a text editor of your own choosing, like vim, nano, and emacs.
"#;

pub(crate) fn init_new_config_driver() -> Result<(), Box<dyn std::error::Error>> {
    println!("{}", MESSAGE_GREETING_CONFIG_INIT);
    println!("{}", MESSAGE_LOCATION_EXPLAINER);

    // default_location_name and default_location are separate bc
    //      default_location_name IS user input
    //      but default_location IS api information based on last substring of default_location_name
    let (default_location_name, default_location) =
        crate::journal::query::user::ask_for_location()?;

    println!("{}", MESSAGE_EDITORS_EXPLAINER);

    let editor = crate::journal::query::user::ask_for_text_editor_multchoice()?;

    let config_contents = format!(
        "[defaults]\n\
        location_full_name=\"{}\"\n\
        location_latitude=\"{}\"\n\
        location_longitude=\"{}\"\n\
        timezone=\"{}\"\n\
        editor=\"{}\"\n",
        default_location_name,
        default_location.latitude,
        default_location.longitude,
        default_location.timezone,
        editor
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
        config_file_pathbuf
            .parent()
            .ok_or(FileError::HomeDirNotFound)?
            .to_string_lossy()
            .as_bytes(),
    )?;

    Ok(())
}

pub(crate) fn create_new_entry_driver() -> Result<(), Box<dyn std::error::Error>> {
    // Check if the dotfile (Always ~/.journal) exists.
    // If it doesn't it execs an early return of Ok(())
    if !crate::journal::file::is_dotfile_exists()? {
        println!(
            "Oops!  Looks like you haven't initialised your journal yet.  Try running `journal init` first."
        );
        return Ok(());
    }

    // This will read the contents of the dotfile, which is the path of the config file, which is set by the user in the init
    let base_dir = crate::journal::file::read_dotfile()?;

    // Retrieve details from config file
    let (location_full_name, location_latitude, location_longitude, timezone, editor ) =
        crate::journal::file::read_configfile(&base_dir)?;

    // Create the file here
    let sample_file_path = crate::journal::calculators::get_path_to_todays_entry(base_dir)?;
    let sample_file_message = format!(
        "This is a sample file. Here are the details for config.toml. You are in {} ({}, {}) in {}, and you want to use {}.\n",
        location_full_name, location_latitude, location_longitude, timezone, editor
    );
    let mut sample_file = std::fs::File::create(sample_file_path)?;

    // TODO Generate journal entry preambles

    // The line that writes var file_message into the file.
    std::io::Write::write_all(&mut sample_file, sample_file_message.as_bytes())?;

    Ok(())
}
