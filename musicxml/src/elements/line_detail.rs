use crate::datatypes::{Color, LineType, StaffLine, Tenths, YesNo};
use musicxml_internal::*;
use musicxml_macros::*;

/// Attributes pertaining to the [LineDetail] element.
#[derive(Debug, PartialEq, Eq, AttributeDeserialize, AttributeSerialize)]
pub struct LineDetailAttributes {
  /// Indicates the staff line affected, numbered from bottom to top.
  pub line: StaffLine,
  /// Indicates the color of an element.
  pub color: Option<Color>,
  /// Specifies if the line is solid, dashed, dotted, or wavy.
  pub line_type: Option<LineType>,
  /// Specifies whether or not to print an object. It is yes if not specified.
  pub print_object: Option<YesNo>,
  /// Used to size and scale an image. The image should be scaled independently in X and Y if both `height` and `width` are specified.
  /// If only `width` is specified, the image should be scaled proportionally to fit in the specified X dimension.
  pub width: Option<Tenths>,
}

/// If the [StaffLines][super::StaffLines] element is present, the appearance of each line may be individually specified with a [LineDetail] element.
///
/// The `print_object` attribute allows lines to be hidden within a staff. This is used in special situations such as a widely-spaced percussion staff
/// where a note placed below the higher line is distinct from a note placed above the lower line. Hidden staff lines are included when specifying clef
/// lines and determining [DisplayStep][super::DisplayStep] / [DisplayOctave][super::DisplayOctave] values, but are not counted as lines for the purposes
/// of the [SystemLayout][super::SystemLayout] and [StaffLayout][super::StaffLayout] elements.
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
#[rename("line-detail")]
pub struct LineDetail {
  /// Element-specific attributes
  pub attributes: LineDetailAttributes,
  /// Element-specific content
  pub content: (),
}
