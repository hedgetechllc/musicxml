use musicxml_internal::*;
use musicxml_macros::*;

/// Multiple [PartLink][super::PartLink] elements can reference different types of linked documents, such as parts and condensed score.
///
/// The optional [GroupLink] elements identify the groups used in the linked document. The content of a [GroupLink] element should match the content
/// of a [Group][super::Group] element in the linked document.
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
#[rename("group-link")]
pub struct GroupLink {
  /// Element-specific attributes
  pub attributes: (),
  /// Element-specific content
  pub content: String,
}
