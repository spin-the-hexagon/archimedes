use eyre::Result;

use crate::error::ADHDError;

/// This method finds a container character that's not listed in the string.
/// Priority goes as the following:
/// 1. The active container (so it may be reused for the sake of efficiency)
/// 2. The following characters: "[]<>()abcdefghijklmnopqrstuvwxyz"
fn determine_container(
    active_container: &Option<char>,
    segment: &String,
) -> Result<char, ADHDError> {
    if let Some(container) = *active_container {
        if !segment.to_lowercase().contains(container) {
            return Ok(container);
        }
    }

    for char in "[]<>()abcdefghijklmnopqrstuvwxyz".chars() {
        if !segment.to_lowercase().contains(char) {
            return Ok(char);
        }
    }

    Err(ADHDError::CharactersExhausted(segment.clone()))
}

/// This method turns a list of strings into a parsable content string.
/// As of the writing of this comment, there's no Rust implementation for parsing this, but the project(ADHD) Scratch client parses this.
/// The format is a bit strange, but here's how it works
///
/// - list of the following (length not known)
///     - optional
///         - "$" character
///         - a character to be used as the new "container"
///     - the list item
///     - the container character
/// - EOF
pub fn vec_to_string(content: Vec<String>) -> Result<String> {
    let mut output = "".to_string();
    let mut container = None;

    let switch_indicator = "$";

    for segment in content {
        let target_container = (determine_container)(&container, &segment)?;

        if Some(target_container) != container || segment.starts_with(&switch_indicator) {
            output += &format!("{switch_indicator}{target_container}");
            container = Some(target_container);
        }

        output += &format!("{segment}{}", container.unwrap());
    }

    Ok(output)
}
