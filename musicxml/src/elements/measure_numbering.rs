use crate::datatypes::{
  Color, FontFamily, FontSize, FontStyle, FontWeight, LeftCenterRight, MeasureNumberingValue, StaffNumber,
  SystemRelationNumber, Tenths, Valign, YesNo,
};
use musicxml_internal::*;
use musicxml_macros::*;

/// Attributes pertaining to the [MeasureNumbering] element.
#[derive(Debug, Default, PartialEq, Eq, AttributeDeserialize, AttributeSerialize)]
pub struct MeasureNumberingAttributes {
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
  /// The `multiple_rest_always` and `multiple_rest_range` attributes describe how measure numbers are shown on multiple rests when the
  /// [MeasureNumbering] value is not set to none. The `multiple_rest_always` attribute is set to yes when the measure number should always
  /// be shown, even if the multiple rest starts midway through a system when measure numbering is set to system level.
  pub multiple_rest_always: Option<YesNo>,
  /// The `multiple_rest_always` and `multiple_rest_range` attributes describe how measure numbers are shown on multiple rests when the
  /// [MeasureNumbering] value is not set to none. The `multiple_rest_range` attribute is set to yes when measure numbers on multiple rests
  /// display the range of numbers for the first and last measure, rather than just the number of the first measure.
  pub multiple_rest_range: Option<YesNo>,
  /// Changes the horizontal position relative to the default position, either as computed by the individual program, or as overridden by the `default_x` attribute.
  /// Positive x is right and negative x is left. It should be interpreted in the context of the [Offset][super::Offset] element or directive attribute if those are present.
  pub relative_x: Option<Tenths>,
  /// Changes the vertical position relative to the default position, either as computed by the individual program, or as overridden by the `default_y` attribute.
  /// Positive y is up and negative y is down. It should be interpreted in the context of the `placement` attribute if that is present.
  pub relative_y: Option<Tenths>,
  /// Refers to staff numbers within the part, from top to bottom on the system. It indicates which staff is used as the reference point for vertical positioning.
  /// A value of 1 is assumed if not present.
  pub staff: Option<StaffNumber>,
  /// Specifies if measure numbers are associated with a system rather than the particular part where the [MeasureNumbering] element appears.
  pub system: Option<SystemRelationNumber>,
  /// Indicates vertical alignment to the top, middle, bottom, or baseline of the text. The default is implementation-dependent.
  pub valign: Option<Valign>,
}

/// The [MeasureNumbering] element describes how frequently measure numbers are displayed on this part.
///
/// ![MeasureNumbering](https://hedgetechllc.github.io/musicxml/musicxml/elements/measure-numbering.png)
///
/// The `text` attribute from the [Measure][super::Measure] element is used for display, or the `number` attribute if the text attribute is not present.
/// Measures with an `implicit` attribute set to yes never display a measure number, regardless of the [MeasureNumbering] setting.
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
#[rename("measure-numbering")]
pub struct MeasureNumbering {
  /// Element-specific attributes
  pub attributes: MeasureNumberingAttributes,
  /// Element-specific content
  pub content: MeasureNumberingValue,
}
