use musicxml_internal::{DatatypeDeserializer, DatatypeSerializer};
use musicxml_macros::{DatatypeDeserialize, DatatypeSerialize};

/// Used to specify repeat directions.
///
/// The start of the repeat has a [Forward][BackwardForward::Forward] direction while
/// the end of the repeat has a [Backward][BackwardForward::Backward] direction.
#[derive(Debug, PartialEq, Eq, DatatypeDeserialize, DatatypeSerialize)]
pub enum BackwardForward {
  /// ![Backward](https://hedgetechllc.github.io/musicxml/musicxml/datatypes/backward-forward-backward.png)
  Backward,
  /// ![Forward](https://hedgetechllc.github.io/musicxml/musicxml/datatypes/backward-forward-forward.png)
  Forward,
}
