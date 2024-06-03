use super::{NumeralFifths, NumeralMode};
use crate::datatypes::YesNo;
use musicxml_internal::*;
use musicxml_macros::*;

/// Attributes pertaining to the [NumeralKey] element.
#[derive(Debug, Default, PartialEq, Eq, AttributeDeserialize, AttributeSerialize)]
pub struct NumeralKeyAttributes {
  /// Specifies whether or not to print an object. It is yes if not specified.
  pub print_object: Option<YesNo>,
}

#[derive(Debug, PartialEq, Eq, ContentDeserialize, ContentSerialize)]
pub struct NumeralKeyContents {
  pub numeral_fifths: NumeralFifths,
  pub numeral_mode: NumeralMode,
}

/// The [NumeralKey] element is used when the key for the numeral is different than the key specified by the key signature.
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
#[rename("numeral-key")]
pub struct NumeralKey {
  /// Element-specific attributes
  pub attributes: NumeralKeyAttributes,
  #[flatten]
  /// Element-specific content
  pub content: NumeralKeyContents,
}