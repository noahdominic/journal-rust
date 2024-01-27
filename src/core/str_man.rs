pub(crate) fn split_location(raw_location: &str) -> Result<&str, &'static str> {
    let city = raw_location
        .rsplit(',')
        .next()
        .ok_or("Invalid raw_location format")?
        .trim();

    Ok(city)
}

pub(crate) fn sanitise_spaces_html(input: &str) -> String {
    input.replace(" ", "%20")
}
