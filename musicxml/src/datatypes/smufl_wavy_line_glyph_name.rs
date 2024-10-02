use super::smufl_glyph_name::SmuflGlyphName;
use alloc::string::{String, ToString};
use core::ops::Deref;
use musicxml_internal::{DatatypeDeserializer, DatatypeSerializer};
use musicxml_macros::DatatypeSerialize;
use regex::Regex;

/// Used to reference a specific Standard Music Font Layout (SMuFL) wavy line character.
///
/// The value is a SMuFL canonical glyph name that either starts with wiggle, or begins
/// with guitar and ends with VibratoStroke. This includes all the glyphs in the Multi-segment
/// lines range, excluding the beam glyphs.
///
/// The value of an instance of this type may be accessed by dereferencing the struct: `*datatype_val`.
#[derive(Debug, PartialEq, Eq, DatatypeSerialize)]
pub struct SmuflWavyLineGlyphName(pub String);

impl Deref for SmuflWavyLineGlyphName {
  type Target = String;
  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl DatatypeDeserializer for SmuflWavyLineGlyphName {
  fn deserialize(value: &str) -> Result<Self, String> {
    if let Ok(token) = SmuflGlyphName::deserialize(value) {
      if Regex::new(r"(^wiggle[a-zA-Z0-9]*$)|(^guitar[a-zA-Z0-9]*VibratoStroke$)")
        .unwrap()
        .is_match(&token)
      {
        Ok(SmuflWavyLineGlyphName((*token).clone()))
      } else {
        Err(format!(
          "Value {value} is invalid for the <smufl-wavy-glyph-name> data type"
        ))
      }
    } else {
      Err(format!(
        "Value {value} is invalid for the <smufl-wavy-glyph-name> data type"
      ))
    }
  }
}

#[cfg(test)]
mod smufl_wavy_line_glyph_name_tests {
  use super::*;

  #[test]
  fn deserialize_valid1() {
    let result = SmuflWavyLineGlyphName::deserialize("wiggleLongLine");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), SmuflWavyLineGlyphName(String::from("wiggleLongLine")));
  }

  #[test]
  fn deserialize_valid2() {
    let result = SmuflWavyLineGlyphName::deserialize("guitarLongVibratoStroke");
    assert!(result.is_ok());
    assert_eq!(
      result.unwrap(),
      SmuflWavyLineGlyphName(String::from("guitarLongVibratoStroke"))
    );
  }

  #[test]
  fn deserialize_invalid1() {
    let result = SmuflWavyLineGlyphName::deserialize("sdsdfwigglesdfs");
    assert!(result.is_err());
  }

  #[test]
  fn deserialize_invalid2() {
    let result = SmuflWavyLineGlyphName::deserialize("guitar");
    assert!(result.is_err());
  }

  #[test]
  fn deserialize_invalid3() {
    let result = SmuflWavyLineGlyphName::deserialize("LongVibrato");
    assert!(result.is_err());
  }
}
