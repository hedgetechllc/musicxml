use musicxml_internal::{DatatypeDeserializer, DatatypeSerializer};
use musicxml_macros::{DatatypeDeserialize, DatatypeSerialize};

/// Describes how measure numbers are displayed on this part: no numbers, numbers every measure, or numbers every system.
#[derive(Debug, PartialEq, Eq, DatatypeDeserialize, DatatypeSerialize)]
pub enum MeasureNumberingValue {
  /// ![None](measure-numbering-value-none.png)
  None,
  /// ![Measure](measure-numbering-value-measure.png)
  Measure,
  /// ![System](measure-numbering-value-system.png)
  System,
}
