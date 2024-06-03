use super::{ExceptVoice, SlashDot, SlashType};
use crate::datatypes::{PositiveInteger, StartStop, YesNo};
use musicxml_internal::*;
use musicxml_macros::*;

/// Attributes pertaining to the [BeatRepeat] element.
#[derive(Debug, PartialEq, Eq, AttributeDeserialize, AttributeSerialize)]
pub struct BeatRepeatAttributes {
  /// Indicates the starting or stopping point of the section displaying the beat repeat symbols.
  pub r#type: StartStop,
  /// Specifies the number of slashes to use in the symbol. The value is 1 if not specified.
  pub slashes: Option<PositiveInteger>,
  /// Indicates whether or not to use dots as well (for instance, with mixed rhythm patterns).
  /// The value is no if not specified.
  pub use_dots: Option<YesNo>,
}

#[derive(Debug, Default, PartialEq, Eq, ContentDeserialize, ContentSerialize)]
pub struct BeatRepeatContents {
  pub slash_type: Option<SlashType>,
  pub slash_dot: Vec<SlashDot>,
  pub except_voice: Vec<ExceptVoice>,
}

/// The [BeatRepeat] element is used to indicate that a single beat (but possibly many notes) is repeated.
/// 
/// ![BeatRepeat](beat-repeat.png)
/// 
/// The "stop" type indicates the first beat where the repeats are no longer displayed. Both the start and stop of the beats being repeated
/// should be specified unless the repeats are displayed through the end of the part.
/// 
/// The [BeatRepeat] element specifies a notation style for repetitions. The actual music being repeated needs to be repeated within the MusicXML file.
/// This element specifies the notation that indicates the repeat.
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
#[rename("beat-repeat")]
pub struct BeatRepeat {
  /// Element-specific attributes
  pub attributes: BeatRepeatAttributes,
  #[flatten]
  /// Element-specific content
  pub content: BeatRepeatContents,
}
