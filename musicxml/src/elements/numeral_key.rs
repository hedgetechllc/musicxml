use super::{NumeralFifths, NumeralMode};
use crate::datatypes::YesNo;
use alloc::{string::String, vec::Vec};
use musicxml_internal::*;
use musicxml_macros::*;

/// Attributes pertaining to the [NumeralKey] element.
#[derive(Debug, Default, PartialEq, Eq, AttributeDeserialize, AttributeSerialize)]
pub struct NumeralKeyAttributes {
  /// Specifies whether or not to print an object. It is yes if not specified.
  pub print_object: Option<YesNo>,
}

/// Contents of the [NumeralKey] element.
#[derive(Debug, PartialEq, Eq, ContentDeserialize, ContentSerialize)]
pub struct NumeralKeyContents {
  /// The [NumeralFifths] element specifies the number of fifths in the key signature of the numeral.
  pub numeral_fifths: NumeralFifths,
  /// The [NumeralMode] element specifies the mode of the key signature of the numeral.
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
