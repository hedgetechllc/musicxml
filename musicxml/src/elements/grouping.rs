use super::Feature;
use crate::datatypes::{Id, StartStopSingle, Token};
use musicxml_internal::*;
use musicxml_macros::*;

/// Attributes pertaining to the [Grouping] element.
#[derive(Debug, PartialEq, Eq, AttributeDeserialize, AttributeSerialize)]
pub struct GroupingAttributes {
  /// Indicates if this is a single-note grouping, or the start or stop of a multi-note grouping.
  pub r#type: StartStopSingle,
  /// Specifies an ID that is unique to the entire document.
  pub id: Option<Id>,
  /// Distinguishes which [Grouping] elements are in which hierarchy.
  pub member_of: Option<Token>,
  /// Distinguishes between various overlapping and hierarchical groupings. The default value is 1.
  pub number: Option<Token>,
}

/// Contents of the [Grouping] element.
#[derive(Debug, Default, PartialEq, Eq, ContentDeserialize, ContentSerialize)]
pub struct GroupingContents {
  /// The [Feature] element is used for musical analysis.
  pub feature: Vec<Feature>,
}

/// The [Grouping] element is used for musical analysis.
///
/// When the `type` attribute is start or single, it usually contains one or more [Feature] elements.
/// Feature elements contained within a stop type of grouping may be ignored.
///
/// This element is flexible to allow for different types of analyses. Future versions of the MusicXML format may add elements that can represent more
/// standardized categories of analysis data, allowing for easier data sharing.
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
pub struct Grouping {
  /// Element-specific attributes
  pub attributes: GroupingAttributes,
  #[flatten]
  /// Element-specific content
  pub content: GroupingContents,
}
