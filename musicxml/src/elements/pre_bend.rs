use musicxml_internal::*;
use musicxml_macros::*;

/// The [PreBend] element indicates that a bend is a pre-bend rather than a normal bend or a release.
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
#[rename("pre-bend")]
pub struct PreBend {
  /// Element-specific attributes
  pub attributes: (),
  /// Element-specific content
  pub content: (),
}
