use crate::datatypes::{FontFamily, FontSize, FontStyle, FontWeight};
use alloc::{string::String, vec::Vec};
use musicxml_internal::*;
use musicxml_macros::*;

/// Attributes pertaining to the [PerMinute] element.
#[derive(Debug, Default, PartialEq, Eq, AttributeDeserialize, AttributeSerialize)]
pub struct PerMinuteAttributes {
  /// A comma-separated list of font names.
  pub font_family: Option<FontFamily>,
  /// One of the CSS sizes or a numeric point size.
  pub font_size: Option<FontSize>,
  /// Normal or italic style.
  pub font_style: Option<FontStyle>,
  /// Normal or bold weight.
  pub font_weight: Option<FontWeight>,
}

/// The [PerMinute] element can be a number, or a text description including numbers.
///
/// If a font is specified, it overrides the font specified for the overall [Metronome][super::Metronome] element.
/// This allows separate specification of a music font for the [BeatUnit][super::BeatUnit] and a text font for the numeric value,
/// in cases where a single metronome font is not used.
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
#[rename("per-minute")]
pub struct PerMinute {
  /// Element-specific attributes
  pub attributes: PerMinuteAttributes,
  /// Element-specific content
  pub content: String,
}
