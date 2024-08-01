use alloc::string::String;
use musicxml_internal::{DatatypeDeserializer, DatatypeSerializer};
use musicxml_macros::{DatatypeDeserialize, DatatypeSerialize};

/// Indicates whether to show a part of a tuplet relating to the
/// [TupletActual][crate::elements::TupletActual] element, both the
/// [TupletActual][crate::elements::TupletActual] or [TupletNormal][crate::elements::TupletNormal]
/// elements, or neither.
#[derive(Debug, PartialEq, Eq, DatatypeDeserialize, DatatypeSerialize)]
pub enum ShowTuplet {
  /// Show only the [TupletActual][crate::elements::TupletActual] element.
  Actual,
  /// Show both the [TupletActual][crate::elements::TupletActual] and [TupletNormal][crate::elements::TupletNormal] elements.
  Both,
  /// Show neither the [TupletActual][crate::elements::TupletActual] nor [TupletNormal][crate::elements::TupletNormal] elements.
  None,
}
