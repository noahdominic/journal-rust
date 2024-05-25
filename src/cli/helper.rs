use crate as journey2;

pub(crate) fn is_journal_initialised() -> Result<bool, journey2::core::file::FileError> {
    let is_journal_initialised = journey2::core::file::is_config_file_exists()?;

    if !is_journal_initialised {
        println!(
            "Oops!  Looks like you haven't initialised your journal yet.  Try running `journal init` first."
        );
    }

    Ok(is_journal_initialised)
}
