// Copyright 2023, 2024  Noah Dominic Miranda Silvio.  All rights reserved.
// Licensed under the EUPL v1.2

//! Contains the basis prompt-functions for user input
//!
//! The basis prompt-functions are the fundamental unit by which
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
///
/// # Note
///
/// This function is heavily reused.  Almost all functions with prefix
/// `prompt_user_for` will call this function.  This function is of
/// critical importance.
pub(in crate::cli::interaction) fn prompt_user_for_string(
    question: &str,
    hint: &str,
) -> std::io::Result<String> {
    let mut user_response = String::new();

    lnprint!("{question} ({hint}): ");
    std::io::stdout().flush()?;
    std::io::stdin().read_line(&mut user_response)?;

    Ok(user_response.trim_end().to_string())
}

pub(in crate::cli::interaction) fn prompt_user_for_choice<'a, T>(
    msg_when_none: &'a str,
    msg_when_many: &'a str,
    choice_prompt: &'a str,
    msg_when_invalid_choice: &'a str,
    msg_when_no_more_chances: &'a str,
    choices: &'a [T],
) -> Result<&'a T, Box<dyn std::error::Error>>
where
    T: std::fmt::Display,
{
    match choices.len() {
        0 => Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            msg_when_none,
        ))),
        1 => Ok(&choices[0]),
        _ => {
            println!("{msg_when_many}");
            for (i, result) in choices.iter().enumerate() {
                println!("{}. {}", i + 1, result);
            }
            let mut chances = 5;
            while chances > 0 {
                let choice = prompt_for_usize(choice_prompt)?;
                if choice > 0 && choice <= choices.len() {
                    return Ok(&choices[choice - 1]);
                } else {
                    println!("{}", msg_when_invalid_choice);
                }
                chances -= 1;
            }
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                msg_when_no_more_chances,
            )));
        }
    }
}

pub(in crate::cli::interaction) fn prompt_for_usize(
    question: &str,
) -> Result<usize, Box<dyn std::error::Error>> {
    let user_response = prompt_user_for_string(question, "[index]")?;
    Ok(user_response.trim().parse::<usize>()?)
}
