use crate::datatypes::{
  AboveBelow, Color, FontFamily, FontSize, FontStyle, FontWeight, Percent, StartNote, Tenths, TrillBeats, TrillStep,
  TwoNoteTurn, YesNo,
};
use musicxml_internal::*;
use musicxml_macros::*;

/// Attributes pertaining to the [DelayedInvertedTurn] element.
#[derive(Debug, Default, PartialEq, Eq, AttributeDeserialize, AttributeSerialize)]
pub struct DelayedInvertedTurnAttributes {
  /// If yes, the trill accelerates during playback. It is no if not specified.
  pub accelerate: Option<YesNo>,
  /// The number of distinct notes during playback, counting the starting note but not the two-note turn. It is 4 if not specified.
  pub beats: Option<TrillBeats>,
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
  /// A comma-separated list of font names.
  pub font_family: Option<FontFamily>,
  /// One of the CSS sizes or a numeric point size.
  pub font_size: Option<FontSize>,
  /// Normal or italic style.
  pub font_style: Option<FontStyle>,
  /// Normal or bold weight.
  pub font_weight: Option<FontWeight>,
  /// The percentage of the way through the duration for landing on the last beat. It is 75 if not specified.
  pub last_beat: Option<Percent>,
  /// Indicates whether something is above or below another element, such as a note or a notation.
  pub placement: Option<AboveBelow>,
  /// Changes the horizontal position relative to the default position, either as computed by the individual program, or as overridden by the `default_x` attribute.
  /// Positive x is right and negative x is left. It should be interpreted in the context of the [Offset][super::Offset] element or directive attribute if those are present.
  pub relative_x: Option<Tenths>,
  /// Changes the vertical position relative to the default position, either as computed by the individual program, or as overridden by the `default_y` attribute.
  /// Positive y is up and negative y is down. It should be interpreted in the context of the `placement` attribute if that is present.
  pub relative_y: Option<Tenths>,
  /// The percentage of the way through the duration for landing on the second beat. It is 25 if not specified.
  pub second_beat: Option<Percent>,
  /// If yes, then a vertical line is used to slash the turn. It is no if not specified.
  pub slash: Option<YesNo>,
  /// The starting note for playback, relative to the current note. It is upper if not specified.
  pub start_note: Option<StartNote>,
  /// The alternating note for playback, relative to the current note. It is whole if not specified.
  pub trill_step: Option<TrillStep>,
  /// Specifies the two-note turn included at the end of the trill, if any. It is none if not specified.
  pub two_note_turn: Option<TwoNoteTurn>,
}

/// The [DelayedInvertedTurn] element indicates an inverted turn that is delayed until the end of the current note.
/// 
/// An inverted turn has the shape which goes down and then up.
/// 
/// ![DelayedInvertedTurn](delayed-inverted-turn.png)
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
#[rename("delayed-inverted-turn")]
pub struct DelayedInvertedTurn {
  /// Element-specific attributes
  pub attributes: DelayedInvertedTurnAttributes,
  /// Element-specific content
  pub content: (),
}
