use super::{Creator, Encoding, Miscellaneous, Relation, Rights, Source};
use musicxml_internal::*;
use musicxml_macros::*;

#[derive(Debug, Default, PartialEq, Eq, ContentDeserialize, ContentSerialize)]
pub struct IdentificationContents {
  pub creator: Vec<Creator>,
  pub rights: Vec<Rights>,
  pub encoding: Option<Encoding>,
  pub source: Option<Source>,
  pub relation: Vec<Relation>,
  pub miscellaneous: Option<Miscellaneous>,
}

/// The [Identification] element contains basic metadata about the score.
/// 
/// It includes information that may apply at a score-wide, movement-wide, or part-wide level. The [Creator], [Rights], [Source],
/// and [Relation] elements are based on [Dublin Core](https://www.dublincore.org/specifications/dublin-core/dcmi-terms/).
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
pub struct Identification {
  /// Element-specific attributes
  pub attributes: (),
  #[flatten]
  /// Element-specific content
  pub content: IdentificationContents,
}
