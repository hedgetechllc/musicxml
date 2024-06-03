use musicxml_internal::{DatatypeDeserializer, DatatypeSerializer};
use musicxml_macros::{DatatypeDeserialize, DatatypeSerialize};
use std::ops::Deref;

/// Represents semitones, used for chromatic alteration.
/// 
/// A value of -1 corresponds to a flat and a value of 1 to a sharp.
/// Decimal values like 0.5 (quarter tone sharp) are used for microtones.
/// 
/// The value of an instance of this type may be accessed by dereferencing the struct: `*datatype_val`.
#[derive(Debug, PartialEq, Eq, DatatypeDeserialize, DatatypeSerialize)]
pub struct Semitones(pub i16);

impl Deref for Semitones {
  type Target = i16;
  fn deref(&self) -> &Self::Target {
    &self.0
  }
}
