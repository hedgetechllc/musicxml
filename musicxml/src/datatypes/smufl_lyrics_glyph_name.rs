use super::smufl_glyph_name::SmuflGlyphName;
use musicxml_internal::{DatatypeDeserializer, DatatypeSerializer};
use musicxml_macros::DatatypeSerialize;
use regex::Regex;
use std::ops::Deref;

/// Used to reference a specific Standard Music Font Layout (SMuFL) lyrics elision character.
///
/// The value is a SMuFL canonical glyph name that starts with lyrics.
///
/// The value of an instance of this type may be accessed by dereferencing the struct: `*datatype_val`.
#[derive(Debug, PartialEq, Eq, DatatypeSerialize)]
pub struct SmuflLyricsGlyphName(pub String);

impl Deref for SmuflLyricsGlyphName {
  type Target = String;
  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl DatatypeDeserializer for SmuflLyricsGlyphName {
  fn deserialize(value: &str) -> Result<Self, String> {
    if let Ok(token) = SmuflGlyphName::deserialize(value) {
      if Regex::new(r"^lyrics.*$").unwrap().is_match(&token) {
        Ok(SmuflLyricsGlyphName((*token).clone()))
      } else {
        Err(format!(
          "Value {} is invalid for the <smufl-lyrics-glyph-name> data type",
          value
        ))
      }
    } else {
      Err(format!(
        "Value {} is invalid for the <smufl-lyrics-glyph-name> data type",
        value
      ))
    }
  }
}

#[cfg(test)]
mod smufl_lyrics_glyph_name_tests {
  use super::*;

  #[test]
  fn deserialize_valid1() {
    let result = SmuflLyricsGlyphName::deserialize("lyrics");
    assert!(result.is_ok());
  }

  #[test]
  fn deserialize_valid2() {
    let result = SmuflLyricsGlyphName::deserialize("lyrics1");
    assert!(result.is_ok());
  }

  #[test]
  fn deserialize_invalid1() {
    let result = SmuflLyricsGlyphName::deserialize("1lyrics");
    assert!(result.is_err());
  }
}
