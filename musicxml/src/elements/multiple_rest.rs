use crate::datatypes::{PositiveInteger, YesNo};
use musicxml_internal::*;
use musicxml_macros::*;

/// Attributes pertaining to the [MultipleRest] element.
#[derive(Debug, Default, PartialEq, Eq, AttributeDeserialize, AttributeSerialize)]
pub struct MultipleRestAttributes {
  /// Specifies whether the multiple rests uses the 1-bar / 2-bar / 4-bar rest symbols,
  /// or a single shape. It is no if not specified.
  pub use_symbols: Option<YesNo>,
}

/// The [MultipleRest] element indicates multiple rests that span several measures.
///
/// ![MultipleRest](https://hedgetechllc.github.io/musicxml/musicxml/elements/multiple-rest.png)
///
/// The element text indicates the number of measures in the multiple rest.
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
#[rename("multiple-rest")]
pub struct MultipleRest {
  /// Element-specific attributes
  pub attributes: MultipleRestAttributes,
  /// Element-specific content
  pub content: PositiveInteger,
}
