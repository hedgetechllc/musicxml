use crate::datatypes::{NmToken, Token, YesNo};
use alloc::{string::String, vec::Vec};
use musicxml_internal::*;
use musicxml_macros::*;

/// Attributes pertaining to the [Supports] element.
#[derive(Debug, PartialEq, Eq, AttributeDeserialize, AttributeSerialize)]
pub struct SupportsAttributes {
  /// Indicates the element that is supported or not by the encoding.
  pub element: NmToken,
  /// If yes, the absence of a particular element with a specified attribute or value is meaningful. It indicates that this information is not present in the score.
  /// If no, the absence is not meaningful because the encoding does not include this type of information.
  pub r#type: YesNo,
  /// Indicates a specific element attribute that is supported or not by the encoding.
  pub attribute: Option<NmToken>,
  /// Indicates a specific attribute value that is supported or not by the encoding. Only used together with the `attribute` attribute.
  pub value: Option<Token>,
}

/// The [Supports] element indicates if a MusicXML encoding supports a particular MusicXML element.
///
/// This is recommended for elements like [Beam][super::Beam], [Stem][super::Stem], and [Accidental][super::Accidental], where the absence
/// of an element is ambiguous if you do not know if the encoding supports that element. It also allows programs to indicate support for
/// specific attributes, or specific attributes with specific values. This lets applications communicate, for example, that all system and/or
/// page breaks are contained in the MusicXML file.
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
pub struct Supports {
  /// Element-specific attributes
  pub attributes: SupportsAttributes,
  /// Element-specific content
  pub content: (),
}
