use super::{Chromatic, Diatonic, Double, OctaveChange};
use crate::datatypes::{Id, StaffNumber};
use musicxml_internal::*;
use musicxml_macros::*;

/// Attributes pertaining to the [Transpose] element.
#[derive(Debug, Default, PartialEq, Eq, AttributeDeserialize, AttributeSerialize)]
pub struct TransposeAttributes {
  /// Specifies an ID that is unique to the entire document.
  pub id: Option<Id>,
  /// Allows a transposition to apply to only the specified staff in the part. If absent, the transposition applies to all staves in the part.
  /// Per-staff transposition is most often used in parts that represent multiple instruments.
  pub number: Option<StaffNumber>,
}

#[derive(Debug, PartialEq, Eq, ContentDeserialize, ContentSerialize)]
pub struct TransposeContents {
  pub diatonic: Option<Diatonic>,
  pub chromatic: Chromatic,
  pub octave_change: Option<OctaveChange>,
  pub double: Option<Double>,
}

/// The [Transpose] element represents what must be added to a written pitch to get a correct sounding pitch.
/// 
/// It is used for encoding parts for transposing instruments that are in written vs. concert pitch.
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
pub struct Transpose {
  /// Element-specific attributes
  pub attributes: TransposeAttributes,
  #[flatten]
  /// Element-specific content
  pub content: TransposeContents,
}
