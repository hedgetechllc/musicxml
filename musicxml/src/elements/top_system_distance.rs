use crate::datatypes::Tenths;
use musicxml_internal::*;
use musicxml_macros::*;

/// The [TopSystemDistance] is measured from the page's top margin to the top line of the first system.
///
/// It is ignored for all but the first system on a page.
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
#[rename("top-system-distance")]
pub struct TopSystemDistance {
  /// Element-specific attributes
  pub attributes: (),
  /// Element-specific content
  pub content: Tenths,
}
