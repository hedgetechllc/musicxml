use super::smufl_glyph_name::SmuflGlyphName;
use musicxml_internal::{DatatypeDeserializer, DatatypeSerializer};
use musicxml_macros::DatatypeSerialize;
use regex::Regex;
use std::ops::Deref;

/// Used to reference a specific Standard Music Font Layout (SMuFL) percussion pictogram character.
///
/// The value is a SMuFL canonical glyph name that starts with pict.
///
/// The value of an instance of this type may be accessed by dereferencing the struct: `*datatype_val`.
#[derive(Debug, PartialEq, Eq, DatatypeSerialize)]
pub struct SmuflPictogramGlyphName(pub String);

impl Deref for SmuflPictogramGlyphName {
  type Target = String;
  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl DatatypeDeserializer for SmuflPictogramGlyphName {
  fn deserialize(value: &str) -> Result<Self, String> {
    if let Ok(token) = SmuflGlyphName::deserialize(value) {
      if Regex::new(r"^pict.*$").unwrap().is_match(&token) {
        Ok(SmuflPictogramGlyphName((*token).clone()))
      } else {
        Err(format!(
          "Value {} is invalid for the <smufl-pictogram-glyph-name>e data type",
          value
        ))
      }
    } else {
      Err(format!(
        "Value {} is invalid for the <smufl-pictogram-glyph-name> data type",
        value
      ))
    }
  }
}

#[cfg(test)]
mod smufl_pictogram_glyph_name_tests {
  use super::*;

  #[test]
  fn deserialize_valid1() {
    let result = SmuflPictogramGlyphName::deserialize("pict2");
    assert!(result.is_ok());
  }

  #[test]
  fn deserialize_valid2() {
    let result = SmuflPictogramGlyphName::deserialize("picto");
    assert!(result.is_ok());
  }

  #[test]
  fn deserialize_invalid1() {
    let result = SmuflPictogramGlyphName::deserialize("pica");
    assert!(result.is_err());
  }
}
