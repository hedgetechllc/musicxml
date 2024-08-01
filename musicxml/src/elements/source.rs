use alloc::string::String;
use musicxml_internal::*;
use musicxml_macros::*;

/// The [Source] element describes the source for the music that is encoded.
///
/// This is similar to the [Dublin Core source](https://www.dublincore.org/specifications/dublin-core/dcmi-terms/elements11/source/) element.
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
pub struct Source {
  /// Element-specific attributes
  pub attributes: (),
  /// Element-specific content
  pub content: String,
}
