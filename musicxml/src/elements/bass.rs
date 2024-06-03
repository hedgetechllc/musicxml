use super::{BassAlter, BassSeparator, BassStep};
use crate::datatypes::HarmonyArrangement;
use musicxml_internal::*;
use musicxml_macros::*;

/// Attributes pertaining to the [Bass] element.
#[derive(Debug, Default, PartialEq, Eq, AttributeDeserialize, AttributeSerialize)]
pub struct BassAttributes {
  /// Specifies where the bass is displayed relative to what precedes it.
  pub arrangement: Option<HarmonyArrangement>,
}

#[derive(Debug, PartialEq, Eq, ContentDeserialize, ContentSerialize)]
pub struct BassContents {
  pub bass_separator: Option<BassSeparator>,
  pub bass_step: BassStep,
  pub bass_alter: Option<BassAlter>,
}

/// The [Bass] element is used to indicate a bass note in popular music chord symbols, e.g. G/C.
/// 
/// It is generally not used in functional harmony, as inversion is generally not used in pop chord symbols.
/// As with [Root][super::Root], it is divided into step and alter elements, similar to pitches.
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
pub struct Bass {
  /// Element-specific attributes
  pub attributes: BassAttributes,
  #[flatten]
  /// Element-specific content
  pub content: BassContents,
}
