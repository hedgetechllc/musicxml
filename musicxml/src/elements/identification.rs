use super::{Creator, Encoding, Miscellaneous, Relation, Rights, Source};
use musicxml_internal::*;
use musicxml_macros::*;

/// Contents of the [Identification] element.
#[derive(Debug, Default, PartialEq, Eq, ContentDeserialize, ContentSerialize)]
pub struct IdentificationContents {
  /// The [Creator] element specifies the creator of the score.
  pub creator: Vec<Creator>,
  /// The [Rights] element specifies the rights of the score.
  pub rights: Vec<Rights>,
  /// The [Encoding] element specifies the encoding of the score.
  pub encoding: Option<Encoding>,
  /// The [Source] element specifies the source of the score.
  pub source: Option<Source>,
  /// The [Relation] element specifies the relation of the score.
  pub relation: Vec<Relation>,
  /// The [Miscellaneous] element contains any other metadata about the score.
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
