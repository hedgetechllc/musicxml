use musicxml_internal::{DatatypeDeserializer, DatatypeSerializer};
use musicxml_macros::{DatatypeDeserialize, DatatypeSerialize};

/// Used as an attribute of the [Tied][crate::elements::Tied] element to specify where the visual representation
/// of a tie begins and ends.
///
/// A [Tied][crate::elements::Tied] element which joins two notes of the same pitch can be specified with
/// [Start][TiedType::Start] on the first note and [Stop][TiedType::Stop] on the second note. To indicate a note
/// should be undamped, use a single [Tied][crate::elements::Tied] element with [LetRing][TiedType::LetRing].
/// For other ties that are visually attached to a single note, such as a tie leading into or out of a repeated
/// section or coda, use two [Tied][crate::elements::Tied] elements on the same note, one [Start][TiedType::Start]
/// and one [Stop][TiedType::Stop].
///
/// In start-stop cases, ties can add more elements using a [Continue][TiedType::Continue] type. This is
/// typically used to specify the formatting of cross-system ties.
#[derive(Debug, PartialEq, Eq, DatatypeDeserialize, DatatypeSerialize)]
pub enum TiedType {
  /// Start of a tie.
  Start,
  /// End of a tie.
  Stop,
  /// Continuation of a tie, usually used for cross-system formatting.
  Continue,
  /// A tie that indicates an instrument should be undamped.
  #[rename("let-ring")]
  LetRing,
}
