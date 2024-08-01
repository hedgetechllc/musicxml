use super::{Duration, Figure, Footnote, Level};
use crate::datatypes::{
  AboveBelow, Color, FontFamily, FontSize, FontStyle, FontWeight, Id, LeftCenterRight, Tenths, Valign, YesNo,
};
use alloc::{string::String, vec::Vec};
use musicxml_internal::*;
use musicxml_macros::*;

/// Attributes pertaining to the [FiguredBass] element.
#[derive(Debug, Default, PartialEq, Eq, AttributeDeserialize, AttributeSerialize)]
pub struct FiguredBassAttributes {
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
  /// Specifies an ID that is unique to the entire document.
  pub id: Option<Id>,
  /// Specifies whether or not parentheses are put around a symbol for an editorial indication. If not specified, it is left to application defaults.
  pub parentheses: Option<YesNo>,
  /// Indicates whether something is above or below another element, such as a note or a notation.
  pub placement: Option<AboveBelow>,
  /// Controls the printing of an augmentation dot separately from the rest of the note or rest.
  /// This is especially useful for notes that overlap in different voices, or for chord sheets that contain lyrics and chords but no melody.
  /// If `print_object` is set to no, this attribute is also interpreted as being set to no if not present.
  pub print_dot: Option<YesNo>,
  /// Controls the printing of a lyric separately from the rest of the note or rest.
  /// This is especially useful for notes that overlap in different voices, or for chord sheets that contain lyrics and chords but no melody.
  /// If `print_object` is set to no, this attribute is also interpreted as being set to no if not present.
  pub print_lyric: Option<YesNo>,
  /// Specifies whether or not to print an object. It is yes if not specified.
  pub print_object: Option<YesNo>,
  /// Controls whether or not spacing is left for an invisible note or object.
  /// It is used only if no note, dot, or lyric is being printed. The value is yes (leave spacing) if not specified.
  pub print_spacing: Option<YesNo>,
  /// Changes the horizontal position relative to the default position, either as computed by the individual program, or as overridden by the `default_x` attribute.
  /// Positive x is right and negative x is left. It should be interpreted in the context of the [Offset][super::Offset] element or directive attribute if those are present.
  pub relative_x: Option<Tenths>,
  /// Changes the vertical position relative to the default position, either as computed by the individual program, or as overridden by the `default_y` attribute.
  /// Positive y is up and negative y is down. It should be interpreted in the context of the `placement` attribute if that is present.
  pub relative_y: Option<Tenths>,
  /// Indicates vertical alignment to the top, middle, bottom, or baseline of the text. The default is implementation-dependent.
  pub valign: Option<Valign>,
}

/// Contents of the [FiguredBass] element.
#[derive(Debug, Default, PartialEq, Eq, ContentDeserialize, ContentSerialize)]
pub struct FiguredBassContents {
  /// The [Figure] element represents a single figured bass figure.
  pub figure: Vec<Figure>,
  /// The [Duration] element is used to indicate changes of figures under a note. Figures are ordered from top to bottom.
  pub duration: Option<Duration>,
  /// The [Footnote] element specifies editorial information or lyrics content.
  pub footnote: Option<Footnote>,
  /// The [Level] element specifies the editorial level of a score or part.
  pub level: Option<Level>,
}

/// The [FiguredBass] element represents figured bass notation.
///
/// A [FiguredBass] element takes its position from the first regular note (not a grace note or chord note) that follows in score order.
/// The optional [Duration] element is used to indicate changes of figures under a note. Figures are ordered from top to bottom.
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
#[rename("figured-bass")]
pub struct FiguredBass {
  /// Element-specific attributes
  pub attributes: FiguredBassAttributes,
  #[flatten]
  /// Element-specific content
  pub content: FiguredBassContents,
}
