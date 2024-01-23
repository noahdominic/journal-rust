// Copyright 2023, 2024  Noah Dominic Miranda Silvio.  All rights reserved.
// Licensed under the EUPL v1.2

//! Contains primitive prompt functions for user input
//! 
//! Primitive prompts functions are the fundamental unit by which 
//! more complex prompts are built upon.

use std::io::Write;

/// Prints a string with blank line paddings.
/// 
/// An aesthetic choice.  Most prompts look better when prepended with a newline.
macro_rules! lnprint {
    ($($arg:tt)*) => {{
        println!();
        print!($($arg)*);
    }}
}

/// Prints a prompt and parses the respopnse as a string.
/// 
/// # Arguments
/// 
/// * `question` - The 'question' that is printed.  Typically a question.
/// * `hint` - Additional information for the user's comprehension.  
///            This will be placed in brackets and printed after the question.
pub(in crate::cli::interaction) fn prompt_user_for_string(question: &str, hint: &str) -> std::io::Result<String> {
    let mut user_response = String::new();
    
    lnprint!("{question} ({hint})");
    std::io::stdout().flush()?;
    std::io::stdin().read_line(&mut user_response)?;

    Ok(user_response.trim_end().to_string())
}