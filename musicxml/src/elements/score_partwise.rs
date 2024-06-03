use super::{Credit, Defaults, Identification, MovementNumber, MovementTitle, Part, PartList, Work};
use crate::datatypes::Token;
use musicxml_internal::*;
use musicxml_macros::*;

/// Attributes pertaining to the [ScorePartwise] element.
#[derive(Debug, Default, PartialEq, Eq, AttributeDeserialize, AttributeSerialize)]
pub struct ScorePartwiseAttributes {
  /// The `version` attribute was added in Version 1.1 for the [ScorePartwise] and [ScoreTimewise][super::ScoreTimewise]
  /// documents. It provides an easier way to get version information than through the MusicXML public ID. The default value is
  /// 1.0 to make it possible for programs that handle later versions to distinguish earlier version files reliably.
  /// Programs that write MusicXML 1.1 or later files should set this attribute.
  pub version: Option<Token>,
}

#[derive(Debug, PartialEq, Eq, ContentDeserialize, ContentSerialize)]
pub struct ScorePartwiseContents {
  pub work: Option<Work>,
  pub movement_number: Option<MovementNumber>,
  pub movement_title: Option<MovementTitle>,
  pub identification: Option<Identification>,
  pub defaults: Option<Defaults>,
  pub credit: Vec<Credit>,
  pub part_list: PartList,
  pub part: Vec<Part>,
}

/// The [ScorePartwise] element is the root element for a partwise MusicXML score.
/// 
/// It includes score header information followed by a series of [Part] elements with [Measure][super::Measure] elements inside.
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
#[rename("score-partwise")]
pub struct ScorePartwise {
  /// Element-specific attributes
  pub attributes: ScorePartwiseAttributes,
  #[flatten]
  /// Element-specific content
  pub content: ScorePartwiseContents,
}
