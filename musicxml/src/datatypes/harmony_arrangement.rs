use musicxml_internal::{DatatypeDeserializer, DatatypeSerializer};
use musicxml_macros::{DatatypeDeserialize, DatatypeSerialize};

/// Indicates how stacked chords and bass notes are displayed within a harmony element.
///
/// The [Horizontal][HarmonyArrangement::Horizontal] value specifies that the second element
/// appears to the right of the first. The [Vertical][HarmonyArrangement::Vertical] value specifies
/// that the second element appears below the first. The [Diagonal][HarmonyArrangement::Diagonal] value
/// specifies that the second element appears both below and to the right of the first.
#[derive(Debug, PartialEq, Eq, DatatypeDeserialize, DatatypeSerialize)]
pub enum HarmonyArrangement {
  /// ![Horizontal](https://hedgetechllc.github.io/musicxml/musicxml/datatypes/harmony-arrangement-horizontal.png)
  Horizontal,
  /// ![Vertical](https://hedgetechllc.github.io/musicxml/musicxml/datatypes/harmony-arrangement-vertical.png)
  Vertical,
  /// ![Diagonal](https://hedgetechllc.github.io/musicxml/musicxml/datatypes/harmony-arrangement-diagonal.png)
  Diagonal,
}
