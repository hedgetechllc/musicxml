use musicxml_internal::{DatatypeDeserializer, DatatypeSerializer};
use musicxml_macros::{DatatypeDeserialize, DatatypeSerialize};
use std::ops::Deref;

/// Used for attributes that reference a specific Standard Music Font Layout (SMuFL) character.
///
/// The value is a SMuFL canonical glyph name, not a code point. For instance, the value for a
/// standard piano pedal mark would be "keyboardPedalPed", not U+E650.
///
/// The value of an instance of this type may be accessed by dereferencing the struct: `*datatype_val`.
#[derive(Debug, PartialEq, Eq, DatatypeDeserialize, DatatypeSerialize)]
pub struct SmuflGlyphName(pub String);

impl Deref for SmuflGlyphName {
  type Target = String;
  fn deref(&self) -> &Self::Target {
    &self.0
  }
}
