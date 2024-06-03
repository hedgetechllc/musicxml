use super::{GroupLink, InstrumentLink};
use crate::datatypes::{AnyUri, Token, XlinkActuate, XlinkShow, XlinkType};
use musicxml_internal::*;
use musicxml_macros::*;

/// Attributes pertaining to the [PartLink] element.
#[derive(Debug, PartialEq, Eq, AttributeDeserialize, AttributeSerialize)]
pub struct PartLinkAttributes {
  /// The `xlink_href` attribute provides the data that allows an application to find a remote resource or resource fragment. See the definition
  /// in the [XML Linking Language recommendation](https://www.w3.org/TR/xlink11/#link-locators).
  pub xlink_href: AnyUri,
  /// The `xlink_actuate` attribute is used to communicate the desired timing of traversal from the starting resource to the ending resource.
  /// The default value is "onRequest". See the definition in the [XML Linking Language recommendation](https://www.w3.org/TR/xlink11/#link-behaviors).
  pub xlink_actuate: Option<XlinkActuate>,
  /// The `xlink_role` attribute indicates a property of the link. See the definition in the
  /// [XML Linking Language recommendation](https://www.w3.org/TR/xlink11/#link-semantics).
  pub xlink_role: Option<Token>,
  /// The `xlink_show` attribute is used to communicate the desired presentation of the ending resource on traversal from the starting resource.
  /// The default value is "replace". See the definition in the [XML Linking Language recommendation](https://www.w3.org/TR/xlink11/#link-behaviors).
  pub xlink_show: Option<XlinkShow>,
  /// The `xlink_title` attribute describes the meaning of a link or resource in a human-readable fashion. See the definition in the
  /// [XML Linking Language recommendation](https://www.w3.org/TR/xlink11/#link-semantics).
  pub xlink_title: Option<Token>,
  /// The `xlink_type` attribute identifies XLink element types. In MusicXML, the value is always "simple". See the definition in the
  /// [XML Linking Language recommendation](https://www.w3.org/TR/xlink11/#link-types).
  pub xlink_type: Option<XlinkType>,
}

#[derive(Debug, Default, PartialEq, Eq, ContentDeserialize, ContentSerialize)]
pub struct PartLinkContents {
  pub instrument_link: Vec<InstrumentLink>,
  pub group_link: Vec<GroupLink>,
}

/// The [PartLink] element allows MusicXML data for both score and parts to be contained within a single compressed MusicXML file.
///
/// It links a [ScorePart][super::ScorePart] from a score document to MusicXML documents that contain parts data. In the case of a single
/// compressed MusicXML file, the link href values are paths that are relative to the root folder of the zip file.
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
#[rename("part-link")]
pub struct PartLink {
  /// Element-specific attributes
  pub attributes: PartLinkAttributes,
  #[flatten]
  /// Element-specific content
  pub content: PartLinkContents,
}
