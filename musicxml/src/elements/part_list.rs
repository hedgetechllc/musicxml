use super::{PartGroup, ScorePart};
use alloc::{string::String, vec::Vec};
use musicxml_internal::*;
use musicxml_macros::*;

/// The [PartListElement] specifies all possible elements available for use in a [PartList] element.
#[derive(Debug, PartialEq, Eq)]
pub enum PartListElement {
  /// The [PartGroup] element indicates a group of parts that is bracketed together.
  PartGroup(PartGroup),
  /// The [ScorePart] element identifies a part in this score.
  ScorePart(ScorePart),
}

/// Contents of the [PartList] element.
#[derive(Debug, PartialEq, Eq)]
pub struct PartListContents {
  /// Ordered list of [PartList] content elements.
  pub content: Vec<PartListElement>,
}

impl ContentDeserializer for PartListContents {
  fn deserialize(elements: &[XmlElement]) -> Result<Self, String> {
    let mut content = PartListContents { content: Vec::new() };
    for element in elements {
      if element.name == "part-group" {
        content
          .content
          .push(PartListElement::PartGroup(PartGroup::deserialize(element)?));
      } else if element.name == "score-part" {
        content
          .content
          .push(PartListElement::ScorePart(ScorePart::deserialize(element)?));
      } else {
        return Err(format!("Unexpected <part-list> element '{}'", element.name));
      }
    }
    Ok(content)
  }
}

impl ContentSerializer for PartListContents {
  fn serialize(element: &Self) -> Vec<XmlElement> {
    let mut elements: Vec<XmlElement> = Vec::new();
    for el in &element.content {
      match el {
        PartListElement::PartGroup(el) => elements.push(PartGroup::serialize(el)),
        PartListElement::ScorePart(el) => elements.push(ScorePart::serialize(el)),
      }
    }
    elements
  }
}

/// The [PartList] element identifies the different musical parts in this document.
///
/// Each part has an ID that is used later within the musical data. Since parts may be encoded separately and combined later, identification elements are
/// present at both the score and [ScorePart] levels.
///
/// There must be at least one [ScorePart], combined as desired with [PartGroup] elements that indicate braces and brackets. Parts are ordered from
/// top to bottom in a score based on the order in which they appear in the [PartList].
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
#[rename("part-list")]
pub struct PartList {
  /// Element-specific attributes
  pub attributes: (),
  #[flatten]
  /// Element-specific content
  pub content: PartListContents,
}
