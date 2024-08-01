use alloc::string::String;
use musicxml_internal::*;
use musicxml_macros::*;

/// The presence of the [SoundingPitch] element indicates this is the pitch which is heard when playing the harmonic.
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
#[rename("sounding-pitch")]
pub struct SoundingPitch {
  /// Element-specific attributes
  pub attributes: (),
  /// Element-specific content
  pub content: (),
}
