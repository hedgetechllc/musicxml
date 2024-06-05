use crate::datatypes::{
  Color, EndingNumber, FontFamily, FontSize, FontStyle, FontWeight, StartStopDiscontinue, SystemRelation, Tenths, YesNo,
};
use musicxml_internal::*;
use musicxml_macros::*;

/// Attributes pertaining to the [Ending] element.
#[derive(Debug, PartialEq, Eq, AttributeDeserialize, AttributeSerialize)]
pub struct EndingAttributes {
  /// Indicates which times the ending is played, similar to the `time_only` attribute used by other elements.
  /// While this often represents the numeric values for what is under the ending line, it can also indicate whether
  /// an ending is played during a larger dal segno or da capo repeat. Single endings such as "1" or
  /// comma-separated multiple endings such as "1,2" may be used.
  pub number: EndingNumber,
  /// Typically, the start type is associated with the left barline of the first measure in an ending.
  /// The stop and discontinue types are associated with the right barline of the last measure in an ending.
  /// Stop is used when the ending mark concludes with a downward jog, as is typical for first endings.
  /// Discontinue is used when there is no downward jog, as is typical for second endings that do not conclude a piece.
  pub r#type: StartStopDiscontinue,
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
  /// Specifies the length of the ending jog.
  pub end_length: Option<Tenths>,
  /// A comma-separated list of font names.
  pub font_family: Option<FontFamily>,
  /// One of the CSS sizes or a numeric point size.
  pub font_size: Option<FontSize>,
  /// Normal or italic style.
  pub font_style: Option<FontStyle>,
  /// Normal or bold weight.
  pub font_weight: Option<FontWeight>,
  /// Specifies whether or not to print an object. It is yes if not specified.
  pub print_object: Option<YesNo>,
  /// Changes the horizontal position relative to the default position, either as computed by the individual program, or as overridden by the `default_x` attribute.
  /// Positive x is right and negative x is left. It should be interpreted in the context of the [Offset][super::Offset] element or directive attribute if those are present.
  pub relative_x: Option<Tenths>,
  /// Changes the vertical position relative to the default position, either as computed by the individual program, or as overridden by the `default_y` attribute.
  /// Positive y is up and negative y is down. It should be interpreted in the context of the `placement` attribute if that is present.
  pub relative_y: Option<Tenths>,
  /// Distinguishes elements that are associated with a system rather than the particular part where the element appears.
  pub system: Option<SystemRelation>,
  /// An offset that specifies where the start of the ending text appears, relative to the start of the ending line.
  pub text_x: Option<Tenths>,
  /// An offset that specifies where the baseline of ending text appears, relative to the start of the ending line.
  pub text_y: Option<Tenths>,
}

/// The [Ending] element represents multiple (e.g. first and second) endings.
///
/// ![Ending](https://hedgetechllc.github.io/musicxml/musicxml/elements/ending.png)
///
/// The element text is used when the text displayed in the ending is different than what appears in the number attribute.
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
pub struct Ending {
  /// Element-specific attributes
  pub attributes: EndingAttributes,
  /// Element-specific content
  pub content: String,
}
