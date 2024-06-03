use musicxml_internal::*;
use musicxml_macros::*;

/// The [Group] element allows the use of different versions of the part for different purposes.
/// 
/// Typical values include "score," "parts," "sound," and "data". Ordering information can be derived from the ordering within a MusicXML score or opus.
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
pub struct Group {
  /// Element-specific attributes
  pub attributes: (),
  /// Element-specific content
  pub content: String,
}
