use super::{PartClef, PartTranspose};
use crate::datatypes::{Id, StaffNumber};
use musicxml_internal::*;
use musicxml_macros::*;

/// Attributes pertaining to the [ForPart] element.
#[derive(Debug, Default, PartialEq, Eq, AttributeDeserialize, AttributeSerialize)]
pub struct ForPartAttributes {
  /// Specifies an ID that is unique to the entire document.
  pub id: Option<Id>,
  /// Allows a transposition to apply to only the specified staff in the part.
  /// If absent, the transposition applies to all staves in the part.
  /// Per-staff transposition is most often used in parts that represent multiple instruments.
  pub number: Option<StaffNumber>,
}

#[derive(Debug, PartialEq, Eq, ContentDeserialize, ContentSerialize)]
pub struct ForPartContents {
  pub part_clef: Option<PartClef>,
  pub part_transpose: PartTranspose,
}

/// The [ForPart] element is used in a concert score to indicate the transposition for a transposed part created from that score.
///
/// It is only used in score files that contain a [ConcertScore][super::ConcertScore] element in the [Defaults][super::Defaults] element.
/// This allows concert scores with transposed parts to be represented in a single uncompressed MusicXML file.
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
#[rename("for-part")]
pub struct ForPart {
  /// Element-specific attributes
  pub attributes: ForPartAttributes,
  #[flatten]
  /// Element-specific content
  pub content: ForPartContents,
}
