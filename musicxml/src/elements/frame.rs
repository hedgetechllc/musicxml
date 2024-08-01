use super::{FirstFret, FrameFrets, FrameNote, FrameStrings};
use crate::datatypes::{Color, Id, LeftCenterRight, Tenths, Token, Valign};
use alloc::{string::String, vec::Vec};
use musicxml_internal::*;
use musicxml_macros::*;

/// Attributes pertaining to the [Frame] element.
#[derive(Debug, Default, PartialEq, Eq, AttributeDeserialize, AttributeSerialize)]
pub struct FrameAttributes {
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
  /// In cases where text extends over more than one line, horizontal alignment and justify values can be different.
  /// The most typical case is for credits, such as:
  ///
  /// ```text
  /// Words and music by
  ///   Pat Songwriter
  /// ```
  /// Typically this type of credit is aligned to the right, so that the position information refers to the right-most part of the text.
  /// But in this example, the text is center-justified, not right-justified.
  ///
  /// The `halign` attribute is used in these situations. If it is not present, its value is the same as for the `justify` attribute.
  /// For elements where a justify attribute is not allowed, the default is implementation-dependent.
  pub halign: Option<LeftCenterRight>,
  /// Used to size and scale an image. The image should be scaled independently in X and Y if both `height` and `width` are specified.
  /// If only `height` is specified, the image should be scaled proportionally to fit in the specified Y dimension.
  pub height: Option<Tenths>,
  /// Specifies an ID that is unique to the entire document.
  pub id: Option<Id>,
  /// Changes the horizontal position relative to the default position, either as computed by the individual program, or as overridden by the `default_x` attribute.
  /// Positive x is right and negative x is left. It should be interpreted in the context of the [Offset][super::Offset] element or directive attribute if those are present.
  pub relative_x: Option<Tenths>,
  /// Changes the vertical position relative to the default position, either as computed by the individual program, or as overridden by the `default_y` attribute.
  /// Positive y is up and negative y is down. It should be interpreted in the context of the `placement` attribute if that is present.
  pub relative_y: Option<Tenths>,
  /// The unplayed attribute is used to indicate that a note is unplayed, for example, notes in cue notes.
  pub unplayed: Option<Token>,
  /// Indicates vertical alignment to the top, middle, bottom, or baseline of the text. The default is implementation-dependent.
  pub valign: Option<Valign>,
  /// Used to size and scale an image. The image should be scaled independently in X and Y if both `height` and `width` are specified.
  /// If only `width` is specified, the image should be scaled proportionally to fit in the specified X dimension.
  pub width: Option<Tenths>,
}

/// Contents of the [Frame] element.
#[derive(Debug, PartialEq, Eq, ContentDeserialize, ContentSerialize)]
pub struct FrameContents {
  /// The [FrameStrings] element specifies the number of strings on the frame.
  pub frame_strings: FrameStrings,
  /// The [FrameFrets] element specifies the number of frets on the frame.
  pub frame_frets: FrameFrets,
  /// The [FirstFret] element specifies the number of the first fret on the frame.
  pub first_fret: Option<FirstFret>,
  /// The [FrameNote] element specifies the note on a string in the frame.
  pub frame_note: Vec<FrameNote>,
}

/// The [Frame] type represents a frame or fretboard diagram used together with a chord symbol.
///
/// The representation is based on the NIFF guitar grid with additional information. The frame type's `unplayed` attribute indicates
/// what to display above a string that has no associated [FrameNote] element. Typical values are "x" and the empty string. If the attribute
/// is not present, the display of the unplayed string is application-defined.
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
pub struct Frame {
  /// Element-specific attributes
  pub attributes: FrameAttributes,
  #[flatten]
  /// Element-specific content
  pub content: FrameContents,
}
