use super::{PartGroup, ScorePart};
use musicxml_internal::*;
use musicxml_macros::*;

#[derive(Debug, PartialEq, Eq)]
pub struct PartListContents {
  pub part_group: Vec<PartGroup>,
  pub score_part: ScorePart,
  pub additional_part_group: Vec<PartGroup>,
  pub additional_score_part: Vec<ScorePart>,
}

impl ContentDeserializer for PartListContents {
  fn deserialize(elements: &Vec<XmlElement>) -> Result<Self, String> {
    let mut part_group: Vec<PartGroup> = Vec::new();
    let mut score_part: Option<ScorePart> = None;
    let mut additional_part_group: Vec<PartGroup> = Vec::new();
    let mut additional_score_part: Vec<ScorePart> = Vec::new();
    for element in elements {
      if element.name == "part-group" {
        if score_part.is_some() {
          additional_part_group.push(PartGroup::deserialize(element)?);
        } else {
          part_group.push(PartGroup::deserialize(element)?);
        }
      } else if element.name == "score-part" {
        if score_part.is_some() {
          additional_score_part.push(ScorePart::deserialize(element)?);
        } else {
          score_part = Some(ScorePart::deserialize(element)?);
        }
      } else {
        return Err(format!("Unexpected <part-list> element '{}'", element.name));
      }
    }
    Ok(PartListContents {
      part_group,
      score_part: score_part.unwrap(),
      additional_part_group,
      additional_score_part,
    })
  }
}

impl ContentSerializer for PartListContents {
  fn serialize(element: &Self) -> Vec<XmlElement> {
    let mut elements: Vec<XmlElement> = Vec::new();
    for el in &element.part_group {
      elements.push(PartGroup::serialize(el));
    }
    elements.push(ScorePart::serialize(&element.score_part));
    for el in &element.additional_part_group {
      elements.push(PartGroup::serialize(el));
    }
    for el in &element.additional_score_part {
      elements.push(ScorePart::serialize(el));
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
