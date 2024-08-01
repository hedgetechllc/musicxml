use super::{ExceptVoice, SlashDot, SlashType};
use crate::datatypes::{StartStop, YesNo};
use alloc::{string::String, vec::Vec};
use musicxml_internal::*;
use musicxml_macros::*;

/// Attributes pertaining to the [Slash] element.
#[derive(Debug, PartialEq, Eq, AttributeDeserialize, AttributeSerialize)]
pub struct SlashAttributes {
  /// Indicates the starting or stopping point of the section displaying slash notation.
  pub r#type: StartStop,
  /// Indicates whether or not to use dots as well (for instance, with mixed rhythm patterns). The value is "no" if not specified. This attribute only has effect if `use_stems` is "no".
  pub use_dots: Option<YesNo>,
  /// If the slash is on every beat, `use_stems` is "no" (the default). To indicate rhythms but not pitches, `use_stems` is set to "yes".
  pub use_stems: Option<YesNo>,
}

/// Contents of the [Slash] element.
#[derive(Debug, Default, PartialEq, Eq, ContentDeserialize, ContentSerialize)]
pub struct SlashContents {
  /// The [SlashType] element indicates the notation style for the slash notation.
  pub slash_type: Option<SlashType>,
  /// The [SlashDot] element indicates the presence of a dot in the slash notation.
  pub slash_dot: Vec<SlashDot>,
  /// The [ExceptVoice] element indicates that the slash notation is for a different voice.
  pub except_voice: Vec<ExceptVoice>,
}

/// The [Slash] element indicates that slash notation is to be used.
///
/// ![Slash](https://hedgetechllc.github.io/musicxml/musicxml/elements/slash.png)
///
/// The stop type indicates the first beat where slash notation no longer displayed. Both the start and stop of the slash notation should be
/// specified unless the slashes are displayed through the end of the part.
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
pub struct Slash {
  /// Element-specific attributes
  pub attributes: SlashAttributes,
  #[flatten]
  /// Element-specific content
  pub content: SlashContents,
}
