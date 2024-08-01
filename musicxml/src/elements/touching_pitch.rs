use alloc::string::String;
use musicxml_internal::*;
use musicxml_macros::*;

/// The presence of the [TouchingPitch] element indicates this is the pitch at which the string is touched lightly to produce the harmonic.
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
#[rename("touching-pitch")]
pub struct TouchingPitch {
  /// Element-specific attributes
  pub attributes: (),
  /// Element-specific content
  pub content: (),
}
