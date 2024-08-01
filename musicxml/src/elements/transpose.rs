use super::{Chromatic, Diatonic, Double, OctaveChange};
use crate::datatypes::{Id, StaffNumber};
use alloc::{string::String, vec::Vec};
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

/// Contents of the [Transpose] element.
#[derive(Debug, PartialEq, Eq, ContentDeserialize, ContentSerialize)]
pub struct TransposeContents {
  /// The [Diatonic] element specifies the number of diatonic steps to transpose.
  pub diatonic: Option<Diatonic>,
  /// The [Chromatic] element specifies the number of chromatic steps to transpose.
  pub chromatic: Chromatic,
  /// The [OctaveChange] element specifies the number of octaves to transpose.
  pub octave_change: Option<OctaveChange>,
  /// The [Double] element specifies the number of double-accidentals to transpose.
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
