use musicxml_internal::*;
use musicxml_macros::*;

/// The [Diatonic] element specifies the number of pitch steps needed to go from written to sounding pitch.
///
/// This allows for correct spelling of enharmonic transpositions. This value does not include [OctaveChange][super::OctaveChange] values;
/// the values for both elements need to be added to the written pitch to get the correct sounding pitch.
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
pub struct Diatonic {
  /// Element-specific attributes
  pub attributes: (),
  /// Element-specific content
  pub content: i16,
}
