use super::{DegreeAlter, DegreeType, DegreeValue};
use crate::datatypes::YesNo;
use musicxml_internal::*;
use musicxml_macros::*;

/// Attributes pertaining to the [Degree] element.
#[derive(Debug, Default, PartialEq, Eq, AttributeDeserialize, AttributeSerialize)]
pub struct DegreeAttributes {
  /// Specifies whether or not to print an object. It is yes if not specified.
  pub print_object: Option<YesNo>,
}

/// Contents of the [Degree] element.
#[derive(Debug, PartialEq, Eq, ContentDeserialize, ContentSerialize)]
pub struct DegreeContents {
  /// The [DegreeValue] element specifies the degree value for the degree element.
  pub degree_value: DegreeValue,
  /// The [DegreeAlter] element specifies the alteration for the degree value.
  pub degree_alter: DegreeAlter,
  /// The [DegreeType] element specifies the type of the degree element.
  pub degree_type: DegreeType,
}

/// The [Degree] element is used to add, alter, or subtract individual notes in the chord.
///
/// The `print_object` attribute can be used to keep the degree from printing separately when it has already taken into account in the
/// text attribute of the [Kind][super::Kind] element.
///
/// A [Harmony][super::Harmony] with a [Kind][super::Kind] value of "other" can be spelled explicitly by using a series of [Degree] elements together with
/// a [Root][super::Root], [Numeral][super::Numeral], or [Function][super::Function] element.
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
pub struct Degree {
  /// Element-specific attributes
  pub attributes: DegreeAttributes,
  #[flatten]
  /// Element-specific content
  pub content: DegreeContents,
}
