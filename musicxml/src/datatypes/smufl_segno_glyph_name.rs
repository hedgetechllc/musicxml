use super::smufl_glyph_name::SmuflGlyphName;
use musicxml_internal::{DatatypeDeserializer, DatatypeSerializer};
use musicxml_macros::DatatypeSerialize;
use regex::Regex;
use std::ops::Deref;

/// Used to reference a specific Standard Music Font Layout (SMuFL) segno character.
///
/// The value is a SMuFL canonical glyph name that starts with segno.
///
/// The value of an instance of this type may be accessed by dereferencing the struct: `*datatype_val`.
#[derive(Debug, PartialEq, Eq, DatatypeSerialize)]
pub struct SmuflSegnoGlyphName(pub String);

impl Deref for SmuflSegnoGlyphName {
  type Target = String;
  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl DatatypeDeserializer for SmuflSegnoGlyphName {
  fn deserialize(value: &str) -> Result<Self, String> {
    if let Ok(token) = SmuflGlyphName::deserialize(value) {
      if Regex::new(r"^segno.*$").unwrap().is_match(&token) {
        Ok(SmuflSegnoGlyphName((*token).clone()))
      } else {
        Err(format!(
          "Value {} is invalid for the <smufl-segno-glyph-name> data type",
          value
        ))
      }
    } else {
      Err(format!(
        "Value {} is invalid for the <smufl-segno-glyph-name> data type",
        value
      ))
    }
  }
}

#[cfg(test)]
mod smufl_segno_glyph_name_tests {
  use super::*;

  #[test]
  fn deserialize_valid1() {
    let result = SmuflSegnoGlyphName::deserialize("segno");
    assert!(result.is_ok());
  }

  #[test]
  fn deserialize_valid2() {
    let result = SmuflSegnoGlyphName::deserialize("segno1");
    assert!(result.is_ok());
  }

  #[test]
  fn deserialize_invalid1() {
    let result = SmuflSegnoGlyphName::deserialize("1segno");
    assert!(result.is_err());
  }

  #[test]
  fn deserialize_invalid2() {
    let result = SmuflSegnoGlyphName::deserialize("segna");
    assert!(result.is_err());
  }
}
