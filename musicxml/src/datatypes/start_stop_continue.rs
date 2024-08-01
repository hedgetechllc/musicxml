use alloc::string::String;
use musicxml_internal::{DatatypeDeserializer, DatatypeSerializer};
use musicxml_macros::{DatatypeDeserialize, DatatypeSerialize};

/// Used for musical elements that can either start or stop, but also need to refer to an intermediate point in the symbol,
/// as for complex slurs or for formatting of symbols across system breaks.
///
/// The values of [Start][StartStopContinue::Start], [Stop][StartStopContinue::Stop], and [Continue][StartStopContinue::Continue]
/// refer to how an element appears in musical score order, not in MusicXML document order. An element with a
/// [Stop][StartStopContinue::Stop] attribute may precede the corresponding element with a [Start][StartStopContinue::Start]
/// attribute within a MusicXML document. This is particularly common in multi-staff music. For example, the stopping point
/// for a slur may appear in staff 1 before the starting point for the slur appears in staff 2 later in the document.
///
/// When multiple elements with the same tag are used within the same note, their order within the MusicXML document should
/// match the musical score order. For example, a note that marks both the end of one slur and the start of a new slur
/// should have the incoming slur element with a type of [Stop][StartStopContinue::Stop] precede the outgoing slur element
/// with a type of [Start][StartStopContinue::Start].
#[derive(Debug, PartialEq, Eq, DatatypeDeserialize, DatatypeSerialize)]
pub enum StartStopContinue {
  /// Starting point of an element.
  Start,
  /// Stopping point of an element.
  Stop,
  /// Continuation of an element, including system breaks.
  Continue,
}
