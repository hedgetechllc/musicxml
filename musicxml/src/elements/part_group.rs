use super::{
  Footnote, GroupAbbreviation, GroupAbbreviationDisplay, GroupBarline, GroupName, GroupNameDisplay, GroupSymbol,
  GroupTime, Level,
};
use crate::datatypes::{StartStop, Token};
use musicxml_internal::*;
use musicxml_macros::*;

/// Attributes pertaining to the [PartGroup] element.
#[derive(Debug, PartialEq, Eq, AttributeDeserialize, AttributeSerialize)]
pub struct PartGroupAttributes {
  /// Indicates the start or stop of the [PartGroup].
  pub r#type: StartStop,
  /// Distinguishes overlapping and nested [PartGroup] elements, not a sequence of
  /// [PartGroup] elements. The default value is 1.
  pub number: Option<Token>,
}

#[derive(Debug, Default, PartialEq, Eq, ContentDeserialize, ContentSerialize)]
pub struct PartGroupContents {
  pub group_name: Option<GroupName>,
  pub group_name_display: Option<GroupNameDisplay>,
  pub group_abbreviation: Option<GroupAbbreviation>,
  pub group_abbreviation_display: Option<GroupAbbreviationDisplay>,
  pub group_symbol: Option<GroupSymbol>,
  pub group_barline: Option<GroupBarline>,
  pub group_time: Option<GroupTime>,
  pub footnote: Option<Footnote>,
  pub level: Option<Level>,
}

/// The [PartGroup] element indicates groupings of parts in the score, usually indicated by braces and brackets.
///
/// Braces that are used for multi-staff parts should be defined in the [Attributes][super::Attributes] element for that part. The [PartGroup] start element appears before
/// the first [ScorePart][super::ScorePart] in the group. The [PartGroup] stop element appears after the last [ScorePart][super::ScorePart] in the group.
///
/// As with [Parts][super::Part], a [PartGroup] can have a name and abbreviation. Values for the child elements are ignored at the stop of a [PartGroup].
///
/// A [PartGroup] element is not needed for a single multi-staff part. By default, multi-staff parts include a brace symbol and
/// (if appropriate given the [BarStyle][super::BarStyle]) common barlines. The symbol formatting for a multi-staff part can be more fully specified
/// using the [PartSymbol][super::PartSymbol] element.
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
#[rename("part-group")]
pub struct PartGroup {
  /// Element-specific attributes
  pub attributes: PartGroupAttributes,
  #[flatten]
  /// Element-specific content
  pub content: PartGroupContents,
}
