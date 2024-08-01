use crate::datatypes::{
  Color, FontFamily, FontSize, FontStyle, FontWeight, KindValue, LeftCenterRight, Tenths, Token, Valign, YesNo,
};
use alloc::{string::String, vec::Vec};
use musicxml_internal::*;
use musicxml_macros::*;

/// Attributes pertaining to the [Kind] element.
#[derive(Debug, Default, PartialEq, Eq, AttributeDeserialize, AttributeSerialize)]
pub struct KindAttributes {
  /// The `bracket_degrees`` attribute is yes if all the degrees should be in a bracket. The default is implementation-dependent.
  pub bracket_degrees: Option<YesNo>,
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
  /// The `parentheses_degrees` attribute is yes if all the degrees should be in parentheses. The default is implementation-dependent.
  pub parentheses_degrees: Option<YesNo>,
  /// Changes the horizontal position relative to the default position, either as computed by the individual program, or as overridden by the `default_x` attribute.
  /// Positive x is right and negative x is left. It should be interpreted in the context of the [Offset][super::Offset] element or directive attribute if those are present.
  pub relative_x: Option<Tenths>,
  /// Changes the vertical position relative to the default position, either as computed by the individual program, or as overridden by the `default_y` attribute.
  /// Positive y is up and negative y is down. It should be interpreted in the context of the `placement` attribute if that is present.
  pub relative_y: Option<Tenths>,
  /// If yes, the [Degree][super::Degree] elements should be stacked above each other. The default is implementation-dependent.
  pub stack_degrees: Option<YesNo>,
  /// Describes how the [Kind] should be spelled in a score. If the `use_symbols` attribute is yes, this value follows the symbol.
  /// The default is implementation-dependent.
  pub text: Option<Token>,
  /// The `use_symbols` attribute is yes if the [Kind] should be represented when possible with harmony symbols rather than letters and numbers. These symbols include:
  ///
  /// - major: a triangle, like Unicode 25B3
  /// - minor: -, like Unicode 002D
  /// - augmented: +, like Unicode 002B
  /// - diminished: °, like Unicode 00B0
  /// - half-diminished: ø, like Unicode 00F8
  ///
  /// The default is implementation-dependent.
  pub use_symbols: Option<YesNo>,
  /// Indicates vertical alignment to the top, middle, bottom, or baseline of the text. The default is implementation-dependent.
  pub valign: Option<Valign>,
}

/// The [Kind] element indicates the type of chord.
///
/// The [Degree][super::Degree] elements can then add, subtract, or alter from these starting points.
///
/// The attributes are used to indicate the formatting of the symbol. Since the [Kind] element is the constant in all the harmony-chord element groups
/// that can make up a polychord, many formatting attributes are here. The alignment attributes are for the entire harmony-chord group of which this kind element is a part.
///
/// For the major-minor [Kind], only the minor symbol is used when `use_symbols` is yes. The major symbol is set using the `symbol` attribute in the
/// [DegreeValue][super::DegreeValue] element. The corresponding [DegreeAlter][super::DegreeAlter] value will usually be 0 in this case.
///
/// The `text` attribute may use strings such as "13sus" that refer to both the kind and one or more [Degree][super::Degree] elements. In this case,
/// the corresponding [Degree][super::Degree] elements should have the `print_object` attribute set to "no" to keep redundant alterations from being displayed.
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
pub struct Kind {
  /// Element-specific attributes
  pub attributes: KindAttributes,
  /// Element-specific content
  pub content: KindValue,
}
