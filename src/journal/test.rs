
#[cfg(test)]
mod test{
    #[test]
    fn test_get_direction() {
        // Test common directions
        assert_eq!(crate::journal::calculators::get_direction(0.0), "N");
        assert_eq!(crate::journal::calculators::get_direction(45.0), "NE");
        assert_eq!(crate::journal::calculators::get_direction(90.0), "E");
        assert_eq!(crate::journal::calculators::get_direction(135.0), "SE");
        assert_eq!(crate::journal::calculators::get_direction(180.0), "S");
        assert_eq!(crate::journal::calculators::get_direction(225.0), "SW");
        assert_eq!(crate::journal::calculators::get_direction(270.0), "W");
        assert_eq!(crate::journal::calculators::get_direction(315.0), "NW");
        assert_eq!(crate::journal::calculators::get_direction(360.0), "N");

        // Test edge cases
        assert_eq!(crate::journal::calculators::get_direction(-45.0), "N");  // Negative degrees
        assert_eq!(crate::journal::calculators::get_direction(361.0), "N");  // Degrees greater than 360
        assert_eq!(crate::journal::calculators::get_direction(22.5), "NNE"); // Degrees on a boundary between two directions
        assert_eq!(crate::journal::calculators::get_direction(11.25), "NNE");  // Degrees on the boundary between N and NNE
        assert_eq!(crate::journal::calculators::get_direction(348.75), "N");  // Degrees on the boundary between NNW and N
    }
}
