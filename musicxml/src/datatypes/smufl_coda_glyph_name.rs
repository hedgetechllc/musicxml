use super::smufl_glyph_name::SmuflGlyphName;
use alloc::string::{String, ToString};
use core::ops::Deref;
use musicxml_internal::{DatatypeDeserializer, DatatypeSerializer};
use musicxml_macros::DatatypeSerialize;
use regex::Regex;

/// Used to reference a specific Standard Music Font Layout (SMuFL) coda character.
///
/// The value is a SMuFL canonical glyph name that starts with coda.
///
/// The value of an instance of this type may be accessed by dereferencing the struct: `*datatype_val`.
#[derive(Debug, PartialEq, Eq, DatatypeSerialize)]
pub struct SmuflCodaGlyphName(pub String);

impl Deref for SmuflCodaGlyphName {
  type Target = String;
  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl DatatypeDeserializer for SmuflCodaGlyphName {
  fn deserialize(value: &str) -> Result<Self, String> {
    if let Ok(token) = SmuflGlyphName::deserialize(value) {
      if Regex::new(r"^coda.*$").unwrap().is_match(&token) {
        Ok(SmuflCodaGlyphName((*token).clone()))
      } else {
        Err(format!(
          "Value {} is invalid for the <smufl-coda-glyph-name> data type",
          value
        ))
      }
    } else {
      Err(format!(
        "Value {} is invalid for the <smufl-coda-glyph-name> data type",
        value
      ))
    }
  }
}

#[cfg(test)]
mod smufl_coda_glyph_name_tests {
  use super::*;

  #[test]
  fn deserialize_valid1() {
    let result = SmuflCodaGlyphName::deserialize("coda1");
    assert!(result.is_ok());
  }

  #[test]
  fn deserialize_valid2() {
    let result = SmuflCodaGlyphName::deserialize("codarepeat");
    assert!(result.is_ok());
  }

  #[test]
  fn deserialize_invalid1() {
    let result = SmuflCodaGlyphName::deserialize("firstcoda");
    assert!(result.is_err());
  }

  #[test]
  fn deserialize_invalid2() {
    let result = SmuflCodaGlyphName::deserialize("mcodam");
    assert!(result.is_err());
  }
}
