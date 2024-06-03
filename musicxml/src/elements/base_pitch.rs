use musicxml_internal::*;
use musicxml_macros::*;

/// The presence of the [BasePitch] element indicates this is the pitch at which the string is played before touching to create the harmonic.
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
#[rename("base-pitch")]
pub struct BasePitch {
  /// Element-specific attributes
  pub attributes: (),
  /// Element-specific content
  pub content: (),
}
