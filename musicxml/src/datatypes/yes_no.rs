use musicxml_internal::{DatatypeDeserializer, DatatypeSerializer};
use musicxml_macros::{DatatypeDeserialize, DatatypeSerialize};

/// Used for boolean-like attributes.
/// 
/// We cannot use W3C XML Schema booleans due to their restrictions on expression of boolean values.
#[derive(Debug, PartialEq, Eq, DatatypeDeserialize, DatatypeSerialize)]
pub enum YesNo {
  /// True.
  Yes,
  /// False.
  No,
}
