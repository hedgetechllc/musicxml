use musicxml_internal::{DatatypeDeserializer, DatatypeSerializer};
use musicxml_macros::{DatatypeDeserialize, DatatypeSerialize};

/// Used for an attribute of musical elements that can either start or stop, such as tuplets.
///
/// The values of [Start][StartStop::Start] and [Stop][StartStop::Stop] refer to how an element appears
/// in musical score order, not in MusicXML document order. An element with a [Stop][StartStop::Stop] attribute
/// may precede the corresponding element with a [Start][StartStop::Start] attribute within a MusicXML document.
/// This is particularly common in multi-staff music. For example, the stopping point for a tuplet may appear in
/// staff 1 before the starting point for the tuplet appears in staff 2 later in the document.
///
/// When multiple elements with the same tag are used within the same note, their order within the MusicXML
/// document should match the musical score order.
#[derive(Debug, PartialEq, Eq, DatatypeDeserialize, DatatypeSerialize)]
pub enum StartStop {
  /// Starting point of an element.
  Start,
  /// Stopping point of an element.
  Stop,
}
