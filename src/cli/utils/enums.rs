pub(crate) enum HelperMessage {
    TutorialWelcome,
    TutorialLocation,
    TutorialEditor,
}

impl std::fmt::Display for HelperMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HelperMessage::TutorialWelcome => write!(
                f,
                r#"
--Welcome to Journey!--

This command-line interface app is here to help you document your thoughts,
experiences, and ideas effortlessly.  Let's get you started :)
"#
            ),
            HelperMessage::TutorialLocation => write!(
                f,
                r#"
--Set your usual location--

Your journal will use your default location to automatically detect your
default time zone and to detect the current weather.  This will also be printed
in your entries.  To ensure the best results, make sure that the last part of
your location is somewhere that is specific enough for accurate time zone and
weather data.

Examples:
  Avenida 9 SO - Carchi, Guayaquil
  1600 Pennsylvania Avenue NW, Washington, D.C
  Lor Marzuki, Singapore City
  Al Quds Open University, Gaza
  25 Paddington Grn, City of Westminster
"#
            ),
            HelperMessage::TutorialEditor => write!(
                f,
                r#"
--Set your editor--

Journey lets you use your preferred text editor, such as vim, nano, or emacs.
"#
            ),
        }
    }
}

// Yes, I'm aware this isn't an enum or a data structure, but it IS a getter to a data structure
pub fn get_weather_map() -> std::collections::HashMap<usize, &'static str> {
    std::collections::HashMap::from([
        (0, "Clear skies"),
        (1, "Mainly clear skies"),
        (2, "Partly cloudy skies"),
        (3, "Overcast skies"),
        (45, "Fog"),
        (48, "Fog"),
        (51, "Light drizzle"),
        (53, "Moderate drizzle"),
        (55, "Heavy drizzle"),
        (56, "Light drizzle, freezing"),
        (57, "Moderate or heavy drizzle, freezing"),
        (61, "Light rain"),
        (63, "Moderate rain"),
        (65, "Heavy rain"),
        (66, "Light rain, freezing"),
        (67, "Moderate or heavy rain, freezing"),
        (71, "Snow fall: Slight intensity"),
        (73, "Snow fall: Moderate intensity"),
        (75, "Snow fall: Heavy intensity"),
        (77, "Snow grains"),
        (80, "Light rain showers"),
        (81, "Moderate rain showers"),
        (82, "Violent rain showers"),
        (85, "Snow showers: Slight intensity"),
        (86, "Snow showers: Heavy intensity"),
        (95, "Thunderstorm: Slight or moderate"),
        (96, "Thunderstorm with slight hail"),
        (99, "Thunderstorm with heavy hail"),
    ])
}
