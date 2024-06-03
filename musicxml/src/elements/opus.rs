use crate::datatypes::{AnyUri, Token, XlinkActuate, XlinkShow, XlinkType};
use musicxml_internal::*;
use musicxml_macros::*;

/// Attributes pertaining to the [Opus] element.
#[derive(Debug, PartialEq, Eq, AttributeDeserialize, AttributeSerialize)]
pub struct OpusAttributes {
  /// The `xlink_href` attribute provides the data that allows an application to find a remote resource or resource fragment.
  /// See the definition in the [XML Linking Language recommendation](https://www.w3.org/TR/xlink11/#link-locators).
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

/// The [Opus] element represents a link to a MusicXML opus document that composes multiple MusicXML scores into a collection.
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
pub struct Opus {
  /// Element-specific attributes
  pub attributes: OpusAttributes,
  /// Element-specific content
  pub content: (),
}
