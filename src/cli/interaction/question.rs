pub(in super::cli::interaction) -> Result<(String), Box<dyn std::error::Error>> {
    let location_full_address = super::primitive::prompt_user_for_string("What is your usual location?", "[optional address specifiers], <location>")?;

    println!("{}", location_full_address);
}