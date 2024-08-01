use super::{Capo, LineDetail, StaffLines, StaffSize, StaffTuning, StaffType};
use crate::datatypes::{ShowFrets, StaffNumber, YesNo};
use alloc::{string::String, vec::Vec};
use musicxml_internal::*;
use musicxml_macros::*;

/// Attributes pertaining to the [StaffDetails] element.
#[derive(Debug, Default, PartialEq, Eq, AttributeDeserialize, AttributeSerialize)]
pub struct StaffDetailsAttributes {
  /// Specifies the staff number from top to bottom within the part. The value is 1 if not present.
  pub number: Option<StaffNumber>,
  /// Specifies whether or not to print an object. It is yes if not specified.
  pub print_object: Option<YesNo>,
  /// Controls whether or not spacing is left for an invisible note or object. It is used only if no note, dot, or lyric is being printed. The value is yes (leave spacing) if not specified.
  pub print_spacing: Option<YesNo>,
  /// Indicates whether to show tablature frets as numbers (0, 1, 2) or letters (a, b, c). It is numbers if not specified.
  pub show_frets: Option<ShowFrets>,
}

/// Contents of the [StaffDetails] element.
#[derive(Debug, Default, PartialEq, Eq, ContentDeserialize, ContentSerialize)]
pub struct StaffDetailsContents {
  /// The [StaffType] element indicates the type of staff.
  pub staff_type: Option<StaffType>,
  /// The [StaffLines] element indicates the number of lines in the staff.
  pub staff_lines: Option<StaffLines>,
  /// The [LineDetail] element indicates the line type of the staff.
  pub line_detail: Vec<LineDetail>,
  /// The [StaffTuning] element indicates the tuning of a single string on a tablature staff.
  pub staff_tuning: Vec<StaffTuning>,
  /// The [Capo] element indicates the fret at which a capo should be placed.
  pub capo: Option<Capo>,
  /// The [StaffSize] element indicates the size of the staff in tenths.
  pub staff_size: Option<StaffSize>,
}

/// The [StaffDetails] element is used to indicate different types of staves.
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
#[rename("staff-details")]
pub struct StaffDetails {
  /// Element-specific attributes
  pub attributes: StaffDetailsAttributes,
  #[flatten]
  /// Element-specific content
  pub content: StaffDetailsContents,
}
