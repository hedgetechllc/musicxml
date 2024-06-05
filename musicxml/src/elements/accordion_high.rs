use musicxml_internal::*;
use musicxml_macros::*;

/// The [AccordionHigh] element indicates the presence of a dot in the high (4') section of the registration symbol.
///
/// ![AccordionHigh](https://hedgetechllc.github.io/musicxml/musicxml/elements/accordion-high.png)
///
/// This element is omitted if no dot is present.
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
#[rename("accordion-high")]
pub struct AccordionHigh {
  /// Element-specific attributes
  pub attributes: (),
  /// Element-specific content
  pub content: (),
}
