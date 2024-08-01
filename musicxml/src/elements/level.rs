use crate::datatypes::{StartStopSingle, SymbolSize, YesNo};
use alloc::{string::String, vec::Vec};
use musicxml_internal::*;
use musicxml_macros::*;

/// Attributes pertaining to the [Level] element.
#[derive(Debug, Default, PartialEq, Eq, AttributeDeserialize, AttributeSerialize)]
pub struct LevelAttributes {
  /// Specifies whether or not brackets are put around a symbol for an editorial indication. If not specified, it is left to application defaults.
  pub bracket: Option<YesNo>,
  /// Specifies whether or not parentheses are put around a symbol for an editorial indication. If not specified, it is left to application defaults.
  pub parentheses: Option<YesNo>,
  /// If the `reference` attribute is yes, this indicates editorial information that is for display only and should not affect playback.
  /// For instance, a modern edition of older music may set reference="yes" on the attributes containing the music's original clef,
  /// key, and time signature. It is no if not specified.
  pub reference: Option<YesNo>,
  /// Specifies the symbol size to use for an editorial indication. If not specified, it is left to application defaults.
  pub size: Option<SymbolSize>,
  /// Indicates whether the editorial information applies to the start of a series of symbols, the end of a series of symbols, or a single symbol.
  /// It is single if not specified for compatibility with earlier MusicXML versions.
  pub r#type: Option<StartStopSingle>,
}

/// The [Level] element is used to specify editorial information for different MusicXML elements.
///
/// The content contains identifying and/or descriptive text about the editorial status of the parent element.
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
pub struct Level {
  /// Element-specific attributes
  pub attributes: LevelAttributes,
  /// Element-specific content
  pub content: String,
}
