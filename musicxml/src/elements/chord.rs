use musicxml_internal::*;
use musicxml_macros::*;

/// The [Chord] element indicates that this note is an additional chord tone with the preceding note.
///
/// ![Chord](https://hedgetechllc.github.io/musicxml/musicxml/elements/chord.png)
///
/// The [Duration][super::Duration] of a [Chord] note does not move the musical position within a [Measure][super::Measure].
/// That is done by the [Duration][super::Duration] of the first preceding note without a [Chord] element.
/// Thus the [Duration][super::Duration] of a [Chord] note cannot be longer than the preceding note.
///
/// In most cases, the [Duration][super::Duration] will be the same as the preceding note. However it can be shorter in situations
/// such as multiple stops for string instruments. Here is an example from Mozart's Concerto No. 3 for Violin, K. 216:
///
/// ![MozartConcertNo3](https://hedgetechllc.github.io/musicxml/musicxml/elements/chord-multiple-stop.png)
///
/// If these first three notes are represented as a chord, the quarter notes must be the ones with the [Chord] element.
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
pub struct Chord {
  /// Element-specific attributes
  pub attributes: (),
  /// Element-specific content
  pub content: (),
}
