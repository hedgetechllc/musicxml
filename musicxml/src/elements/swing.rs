use super::{First, Second, Straight, SwingStyle, SwingType};
use musicxml_internal::*;
use musicxml_macros::*;

/// Contents of the [Swing] element.
#[derive(Debug, Default, PartialEq, Eq, ContentDeserialize, ContentSerialize)]
pub struct SwingContents {
  /// The [Straight] element specifies whether or not to use straight playback.
  pub straight: Option<Straight>,
  /// The [First] and [Second] elements specify the ratio between durations of consecutive notes.
  pub first: Option<First>,
  /// The [First] and [Second] elements specify the ratio between durations of consecutive notes.
  pub second: Option<Second>,
  /// The [SwingType] element specifies the type of swing to use.
  pub swing_type: Option<SwingType>,
  /// The [SwingStyle] element specifies the style of swing to use.
  pub swing_style: Option<SwingStyle>,
}

/// The [Swing] element specifies whether or not to use swing playback, where consecutive on-beat / off-beat eighth or 16th notes are played with unequal nominal durations.
///
/// The [First] and [Second] elements are positive integers that specify the ratio between durations of consecutive notes. For example, a [First] element
/// with a value of 2 and a [Second] element with a value of 1 applied to eighth notes specifies a quarter note / eighth note tuplet playback,
/// where the first note is twice as long as the second note. Ratios should be specified with the smallest integers possible. For example, a ratio
/// of 6 to 4 should be specified as 3 to 2 instead.
///
/// The [Swing] element has no effect for playback of grace notes, notes where a [Type][super::Type] element is not present, and notes where the
/// specified [Duration][super::Duration] is different than the nominal value associated with the specified [Type][super::Type]. If a swung note has
/// `attack` and `release` attributes, those values modify the swung playback.
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
pub struct Swing {
  /// Element-specific attributes
  pub attributes: (),
  #[flatten]
  /// Element-specific content
  pub content: SwingContents,
}
