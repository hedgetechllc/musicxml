use alloc::string::String;
use musicxml_internal::{DatatypeDeserializer, DatatypeSerializer};
use musicxml_macros::{DatatypeDeserialize, DatatypeSerialize};

/// Indicates how to display the arrangement between the [Beats][crate::elements::Beats] and
/// [BeatType][crate::elements::BeatType] values in a time signature.
///
/// The default value is [None][TimeSeparator::None]. The [Horizontal][TimeSeparator::Horizontal],
/// [Diagonal][TimeSeparator::Diagonal], and [Vertical][TimeSeparator::Vertical] values represent
/// horizontal, diagonal lower-left to upper-right, and vertical lines respectively. For these values, the
/// [Beats][crate::elements::Beats] and [BeatType][crate::elements::BeatType] values are arranged on either
/// side of the separator line. The [None][TimeSeparator::None] value represents no separator with the beats
/// and beat-type arranged vertically. The [Adjacent][TimeSeparator::Adjacent] value represents no
/// separator with the beats and beat-type arranged horizontally.
#[derive(Debug, PartialEq, Eq, DatatypeDeserialize, DatatypeSerialize)]
pub enum TimeSeparator {
  /// ![Adjacent](https://hedgetechllc.github.io/musicxml/musicxml/datatypes/time-separator-adjacent.png)
  Adjacent,
  /// ![Diagonal](https://hedgetechllc.github.io/musicxml/musicxml/datatypes/time-separator-diagonal.png)
  Diagonal,
  /// ![Horizontal](https://hedgetechllc.github.io/musicxml/musicxml/datatypes/time-separator-horizontal.png)
  Horizontal,
  /// ![None](https://hedgetechllc.github.io/musicxml/musicxml/datatypes/time-separator-none.png)
  None,
  /// ![Vertical](https://hedgetechllc.github.io/musicxml/musicxml/datatypes/time-separator-vertical.png)
  Vertical,
}
