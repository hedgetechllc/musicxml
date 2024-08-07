use alloc::string::String;
use musicxml_internal::{DatatypeDeserializer, DatatypeSerializer};
use musicxml_macros::{DatatypeDeserialize, DatatypeSerialize};

/// Indicates how to display a time signature.
///
/// The [Normal][TimeSymbol::Normal] value is the usual fractional display, and is the implied
/// symbol type if none is specified. Other options are the [Common][TimeSymbol::Common] and
/// [Cut][TimeSymbol::Cut] time symbols, as well as a [SingleNumber][TimeSymbol::SingleNumber] with
/// an implied denominator. The note symbol indicates that the [BeatType][crate::elements::BeatType]
/// should be represented with the corresponding downstem note rather than a number. The
/// [DottedNote][TimeSymbol::DottedNote] symbol indicates that the [BeatType][crate::elements::BeatType]]
/// should be represented with a dotted downstem note that corresponds to three times the
/// [BeatType][crate::elements::BeatType] value, and a numerator that is one third the
/// [Beats][crate::elements::Beats] value.
#[derive(Debug, PartialEq, Eq, DatatypeDeserialize, DatatypeSerialize)]
pub enum TimeSymbol {
  /// ![Common](https://hedgetechllc.github.io/musicxml/musicxml/datatypes/time-symbol-common.png)
  Common,
  /// ![Cut](https://hedgetechllc.github.io/musicxml/musicxml/datatypes/time-symbol-cut.png)
  Cut,
  /// ![Dotted Note](https://hedgetechllc.github.io/musicxml/musicxml/datatypes/time-symbol-dotted-note.png)
  #[rename("dotted-note")]
  DottedNote,
  /// ![Normal](https://hedgetechllc.github.io/musicxml/musicxml/datatypes/time-symbol-normal.png)
  Normal,
  /// ![Note](https://hedgetechllc.github.io/musicxml/musicxml/datatypes/time-symbol-note.png)
  Note,
  /// ![Single Number](https://hedgetechllc.github.io/musicxml/musicxml/datatypes/time-symbol-single-number.png)
  #[rename("single-number")]
  SingleNumber,
}
