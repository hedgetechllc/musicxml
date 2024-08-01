use crate::datatypes::{Color, GroupSymbolValue, StaffNumber, Tenths};
use alloc::{string::String, vec::Vec};
use musicxml_internal::*;
use musicxml_macros::*;

/// Attributes pertaining to the [PartSymbol] element.
#[derive(Debug, Default, PartialEq, Eq, AttributeDeserialize, AttributeSerialize)]
pub struct PartSymbolAttributes {
  /// Specifies the bottom staff of the symbol when it does not extend across the entire part.
  pub bottom_staff: Option<StaffNumber>,
  /// Indicates the color of an element.
  pub color: Option<Color>,
  /// Changes the computation of the default horizontal position.
  /// The origin is changed relative to the left-hand side of the note or the musical position within the bar.
  /// Positive x is right and negative x is left.
  ///
  /// This attribute provides higher-resolution positioning data than the [Offset][super::Offset] element.
  /// Applications reading a MusicXML file that can understand both features should generally rely on this attribute for its greater accuracy.
  pub default_x: Option<Tenths>,
  /// Changes the computation of the default vertical position.
  /// The origin is changed relative to the top line of the staff. Positive y is up and negative y is down.
  ///
  /// This attribute provides higher-resolution positioning data than the `placement` attribute.
  /// Applications reading a MusicXML file that can understand both attributes should generally rely on this attribute for its greater accuracy.
  pub default_y: Option<Tenths>,
  /// Changes the horizontal position relative to the default position, either as computed by the individual program, or as overridden by the `default_x` attribute.
  /// Positive x is right and negative x is left. It should be interpreted in the context of the [Offset][super::Offset] element or directive attribute if those are present.
  pub relative_x: Option<Tenths>,
  /// Changes the vertical position relative to the default position, either as computed by the individual program, or as overridden by the `default_y` attribute.
  /// Positive y is up and negative y is down. It should be interpreted in the context of the `placement` attribute if that is present.
  pub relative_y: Option<Tenths>,
  /// Specifies the top staff of the symbol when it does not extend across the entire part.
  pub top_staff: Option<StaffNumber>,
}

/// The [PartSymbol] element indicates how a symbol for a multi-staff part is indicated in the score.
///
/// Brace is the default value.
///
/// The `top_staff` and `bottom_staff` attributes are used when the brace does not extend across the entire part. For example, in a 3-staff organ
/// part, the `top_staff` will typically be 1 for the right hand, while the `bottom_staff` will typically be 2 for the left hand. Staff 3 for the pedals
/// is usually outside the brace. By default, the presence of a [PartSymbol] element that does not extend across the entire part also indicates a corresponding
/// change in the common barlines within a part.
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
#[rename("part-symbol")]
pub struct PartSymbol {
  /// Element-specific attributes
  pub attributes: PartSymbolAttributes,
  /// Element-specific content
  pub content: GroupSymbolValue,
}
