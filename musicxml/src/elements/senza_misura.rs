use alloc::string::String;
use musicxml_internal::*;
use musicxml_macros::*;

/// A [SenzaMisura] element explicitly indicates that no time signature is present.
///
/// The optional element content indicates the symbol to be used, if any, such as an X. The [Time][super::Time] element's `symbol`
/// attribute is not used when a [SenzaMisura] element is present.
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
#[rename("senza-misura")]
pub struct SenzaMisura {
  /// Element-specific attributes
  pub attributes: (),
  /// Element-specific content
  pub content: String,
}
