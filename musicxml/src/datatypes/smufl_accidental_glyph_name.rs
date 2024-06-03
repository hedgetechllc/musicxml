use super::smufl_glyph_name::SmuflGlyphName;
use musicxml_internal::{DatatypeDeserializer, DatatypeSerializer};
use musicxml_macros::DatatypeSerialize;
use regex::Regex;
use std::ops::Deref;

/// Used to reference a specific Standard Music Font Layout (SMuFL) accidental character.
///
/// The value is a SMuFL canonical glyph name that starts with one of the strings
/// used at the start of glyph names for SMuFL accidentals.
///
/// The value of an instance of this type may be accessed by dereferencing the struct: `*datatype_val`.
#[derive(Debug, PartialEq, Eq, DatatypeSerialize)]
pub struct SmuflAccidentalGlyphName(pub String);

impl Deref for SmuflAccidentalGlyphName {
  type Target = String;
  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl DatatypeDeserializer for SmuflAccidentalGlyphName {
  fn deserialize(value: &str) -> Result<Self, String> {
    if let Ok(token) = SmuflGlyphName::deserialize(value) {
      if Regex::new(r"^(acc|medRenFla|medRenNatura|medRenShar|kievanAccidental).*$")
        .unwrap()
        .is_match(&token)
      {
        Ok(SmuflAccidentalGlyphName((*token).clone()))
      } else {
        Err(format!(
          "Value {} is invalid for the <smufl-accidental-glyph-name> data type",
          value
        ))
      }
    } else {
      Err(format!(
        "Value {} is invalid for the <smufl-accidental-glyph-name> data type",
        value
      ))
    }
  }
}

#[cfg(test)]
mod smufl_accidental_glyph_name_tests {
  use super::*;

  #[test]
  fn deserialize_valid1() {
    let result = SmuflAccidentalGlyphName::deserialize("medRenFlat");
    assert!(result.is_ok());
  }

  #[test]
  fn deserialize_valid2() {
    let result = SmuflAccidentalGlyphName::deserialize("kievanAccidentalFlat");
    assert!(result.is_ok());
  }

  #[test]
  fn deserialize_invalid1() {
    let result = SmuflAccidentalGlyphName::deserialize("smallmedRenShar");
    assert!(result.is_err());
  }

  #[test]
  fn deserialize_invalid2() {
    let result = SmuflAccidentalGlyphName::deserialize("sacc");
    assert!(result.is_err());
  }

  #[test]
  fn deserialize_invalid3() {
    let result = SmuflAccidentalGlyphName::deserialize("");
    assert!(result.is_err());
  }
}
