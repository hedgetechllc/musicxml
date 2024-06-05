use musicxml_internal::{DatatypeDeserializer, DatatypeSerializer};
use musicxml_macros::{DatatypeDeserialize, DatatypeSerialize};

/// Describes how measure numbers are displayed on this part: no numbers, numbers every measure, or numbers every system.
#[derive(Debug, PartialEq, Eq, DatatypeDeserialize, DatatypeSerialize)]
pub enum MeasureNumberingValue {
  /// ![None](https://hedgetechllc.github.io/musicxml/musicxml/datatypes/measure-numbering-value-none.png)
  None,
  /// ![Measure](https://hedgetechllc.github.io/musicxml/musicxml/datatypes/measure-numbering-value-measure.png)
  Measure,
  /// ![System](https://hedgetechllc.github.io/musicxml/musicxml/datatypes/measure-numbering-value-system.png)
  System,
}
