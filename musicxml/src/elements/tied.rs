use crate::datatypes::{AboveBelow, Color, Divisions, Id, LineType, NumberLevel, OverUnder, StartStopContinue, Tenths};
use musicxml_internal::*;
use musicxml_macros::*;

/// Attributes pertaining to the [Tied] element.
#[derive(Debug, PartialEq, Eq, AttributeDeserialize, AttributeSerialize)]
pub struct TiedAttributes {
  /// Indicates if this is the start, stop, or continuation of a tie, or if this is a tie indicating that an instrument should be undamped.
  pub r#type: StartStopContinue,
  /// The horizontal position of an outgoing bezier point for slurs and ties with a start type, or of an incoming bezier point for slurs and ties with types of stop or continue.
  /// If both the `bezier_x` and `bezier_offset` attributes are present, the `bezier_x` attribute takes priority. This attribute is deprecated as of MusicXML 3.1.
  pub bezier_offset: Option<Divisions>,
  /// The horizontal position of an outgoing bezier point for slurs with a continue type. Not valid for other types. If both the `bezier_x2` and `bezier_offset2` attributes are present,
  /// the `bezier_x2` attribute takes priority. This attribute is deprecated as of MusicXML 3.1.
  pub bezier_offset2: Option<Divisions>,
  /// The horizontal position of an outgoing bezier point for slurs and ties with a start type, or of an incoming bezier point for slurs and ties with types of stop or continue.
  pub bezier_x: Option<Tenths>,
  /// The horizontal position of an outgoing bezier point for slurs with a continue type. Not valid for other types.
  pub bezier_x2: Option<Tenths>,
  /// The vertical position of an outgoing bezier point for slurs and ties with a start type, or of an incoming bezier point for slurs and ties with types of stop or continue.
  pub bezier_y: Option<Tenths>,
  /// The vertical position of an outgoing bezier point for slurs with a continue type. Not valid for other types.
  pub bezier_y2: Option<Tenths>,
  /// Indicates the color of an element.
  pub color: Option<Color>,
  /// The length of dashes in a dashed line. Ignored if the corresponding `line_type` attribute is not dashed.
  pub dash_length: Option<Tenths>,
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
  /// Specifies an ID that is unique to the entire document.
  pub id: Option<Id>,
  /// Specifies if the line is solid, dashed, dotted, or wavy.
  pub line_type: Option<LineType>,
  /// Rarely needed to disambiguate ties, since note pitches will usually suffice. It is available for use in more complex tied notation situations.
  pub number: Option<NumberLevel>,
  /// Indicates whether slurs and ties are overhand (tips down) or underhand (tips up). This is distinct from the `placement` attribute used by any notation type.
  pub orientation: Option<OverUnder>,
  /// Indicates whether something is above or below another element, such as a note or a notation.
  pub placement: Option<AboveBelow>,
  /// Changes the horizontal position relative to the default position, either as computed by the individual program, or as overridden by the `default_x` attribute.
  /// Positive x is right and negative x is left. It should be interpreted in the context of the [Offset][super::Offset] element or directive attribute if those are present.
  pub relative_x: Option<Tenths>,
  /// Changes the vertical position relative to the default position, either as computed by the individual program, or as overridden by the `default_y` attribute.
  /// Positive y is up and negative y is down. It should be interpreted in the context of the `placement` attribute if that is present.
  pub relative_y: Option<Tenths>,
  /// The length of spaces in a dashed line. Ignored if the corresponding `line_type` attribute is not dashed.
  pub space_length: Option<Tenths>,
}

/// The [Tied] element represents the notated tie.
///
/// The [Tie][super::Tie] element represents the tie sound.
///
/// ![Tied](https://hedgetechllc.github.io/musicxml/musicxml/elements/tied.png)
///
/// Ties that join two notes of the same pitch together should be represented with a [Tied] element on the first note with type="start" and a [Tied] element
/// on the second note with type="stop". This can also be done if the two notes being tied are enharmonically equivalent, but have different step values.
/// It is not recommended to use [Tied] elements to join two notes with enharmonically inequivalent pitches.
///
/// Ties that indicate that an instrument should be undamped are specified with a single [Tied] element with type="let-ring".
///
/// Ties that are visually attached to only one note, other than undamped ties, should be specified with two [Tied] elements on the same note,
/// first type="start" then type="stop". This can be used to represent ties into or out of repeated sections or codas.
///
/// When multiple [Tied] elements with the same tag are used within the same note, their order within the MusicXML document should match the musical score order.
/// For example, a note with a tie at the end of a first ending should have the [Tied] element with a type of start precede the [Tied] element with a type of stop.
///
/// Normal ties need only two bezier points: one associated with the start of the tie, the other with the stop. Ties divided over system breaks can specify
/// additional bezier data at [Tied] elements with a continue type.
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
pub struct Tied {
  /// Element-specific attributes
  pub attributes: TiedAttributes,
  /// Element-specific content
  pub content: (),
}
