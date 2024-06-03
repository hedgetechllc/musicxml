use super::{Credit, Defaults, Identification, Measure, MovementNumber, MovementTitle, PartList, Work};
use crate::datatypes::Token;
use musicxml_internal::*;
use musicxml_macros::*;

/// Attributes pertaining to the [ScoreTimewise] element.
#[derive(Debug, Default, PartialEq, Eq, AttributeDeserialize, AttributeSerialize)]
pub struct ScoreTimewiseAttributes {
  /// The `version` attribute was added in Version 1.1 for the [ScorePartwise][super::ScorePartwise] and [ScoreTimewise]
  /// documents. It provides an easier way to get version information than through the MusicXML public ID. The default value is
  /// 1.0 to make it possible for programs that handle later versions to distinguish earlier version files reliably.
  /// Programs that write MusicXML 1.1 or later files should set this attribute.
  pub version: Option<Token>,
}

#[derive(Debug, PartialEq, Eq, ContentDeserialize, ContentSerialize)]
pub struct ScoreTimewiseContents {
  pub work: Option<Work>,
  pub movement_number: Option<MovementNumber>,
  pub movement_title: Option<MovementTitle>,
  pub identification: Option<Identification>,
  pub defaults: Option<Defaults>,
  pub credit: Vec<Credit>,
  pub part_list: PartList,
  pub measure: Vec<Measure>,
}

/// The [ScoreTimewise] element is the root element for a timewise MusicXML score.
///
/// It includes score header information followed by a series of [Measure] elements with [Part][super::Part] elements inside.
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
#[rename("score-timewise")]
pub struct ScoreTimewise {
  /// Element-specific attributes
  pub attributes: ScoreTimewiseAttributes,
  #[flatten]
  /// Element-specific content
  pub content: ScoreTimewiseContents,
}
