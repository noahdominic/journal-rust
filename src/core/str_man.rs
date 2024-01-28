//! Contains functions concerning string manipulation

pub(crate) fn split_location(raw_location: &str) -> &str {
    let city = raw_location
        .rsplit(',')
        .next()
        .unwrap_or(raw_location)
        .trim();

    city
}

pub(crate) fn sanitise_spaces_html(input: &str) -> String {
    input.replace(" ", "%20")
}
