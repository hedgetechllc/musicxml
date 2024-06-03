use musicxml_internal::*;
use musicxml_macros::*;

/// The [AccordionLow] element indicates the presence of a dot in the low (16') section of the registration symbol.
///
/// ![AccordionLow](accordion-low.png)
///
/// This element is omitted if no dot is present.
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
#[rename("accordion-low")]
pub struct AccordionLow {
  /// Element-specific attributes
  pub attributes: (),
  /// Element-specific content
  pub content: (),
}
