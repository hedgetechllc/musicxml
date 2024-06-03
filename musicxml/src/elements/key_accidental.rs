use crate::datatypes::{AccidentalValue, SmuflAccidentalGlyphName};
use musicxml_internal::*;
use musicxml_macros::*;

/// Attributes pertaining to the [KeyAccidental] element.
#[derive(Debug, Default, PartialEq, Eq, AttributeDeserialize, AttributeSerialize)]
pub struct KeyAccidentalAttributes {
  /// Specifies a Standard Music Font Layout (SMuFL) accidental character by its canonical glyph name.
  pub smufl: Option<SmuflAccidentalGlyphName>,
}

/// The [KeyAccidental] element indicates the accidental to be displayed in the key signature, represented in the same manner
/// as the [Accidental][super::Accidental] element.
/// 
/// It is used for disambiguating microtonal accidentals. The different element names indicate the different meaning of altering notes
/// in a scale versus altering a sounding pitch.
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
#[rename("key-accidental")]
pub struct KeyAccidental {
  /// Element-specific attributes
  pub attributes: KeyAccidentalAttributes,
  /// Element-specific content
  pub content: AccidentalValue,
}
