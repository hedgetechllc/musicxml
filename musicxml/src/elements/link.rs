use crate::datatypes::{AnyUri, NmToken, PositiveInteger, Tenths, Token, XlinkActuate, XlinkShow, XlinkType};
use musicxml_internal::*;
use musicxml_macros::*;

/// Attributes pertaining to the [Link] element.
#[derive(Debug, PartialEq, Eq, AttributeDeserialize, AttributeSerialize)]
pub struct LinkAttributes {
  /// The `xlink_href` attribute provides the data that allows an application to find a remote resource or resource fragment.
  /// See the definition in the [XML Linking Language recommendation](https://www.w3.org/TR/xlink11/#link-locators).
  pub xlink_href: AnyUri,
  /// Changes the computation of the default horizontal position.
  /// The origin is changed relative to the left-hand side of the note or the musical position within the bar.
  /// Positive x is right and negative x is left.
  /// 
  /// This attribute provides higher-resolution positioning data than the [Offset][super::Offset] element.
  /// Applications reading a MusicXML file that can understand both features should generally rely on this attribute for its greater accuracy.
  pub default_x: Option<Tenths>,
  /// Changes the computation of the default vertical position.
  /// The origin is changed relative to the top line of the staff. Positive y is up and negative y is down.
  /// 
  /// This attribute provides higher-resolution positioning data than the `placement` attribute.
  /// Applications reading a MusicXML file that can understand both attributes should generally rely on this attribute for its greater accuracy.
  pub default_y: Option<Tenths>,
  /// The `element` attribute specifies an element type for a descendant of the next sibling element that is not a [Link] or
  /// [Bookmark][super::Bookmark] element. When not present, the [Bookmark][super::Bookmark] or [Link] element refers to the next sibling element in the MusicXML file.
  pub element: Option<NmToken>,
  /// The name of this link.
  pub name: Option<Token>,
  /// The `position` attribute specifies the position of the descendant element specified by the `element` attribute, where the first position is 1.
  /// The `position` attribute is ignored if the `element` attribute is not present.
  /// 
  /// For instance, an `element` value of "beam" and a `position` value of "2" defines the [Link] or [Bookmark][super::Bookmark] to refer to the second beam descendant
  /// of the next sibling element that is not a [Link] or [Bookmark][super::Bookmark] element. This is equivalent to an XPath test of `[.//beam[2]]` done in the
  /// context of the sibling element.
  pub position: Option<PositiveInteger>,
  /// Changes the horizontal position relative to the default position, either as computed by the individual program, or as overridden by the `default_x` attribute.
  /// Positive x is right and negative x is left. It should be interpreted in the context of the [Offset][super::Offset] element or directive attribute if those are present.
  pub relative_x: Option<Tenths>,
  /// Changes the vertical position relative to the default position, either as computed by the individual program, or as overridden by the `default_y` attribute.
  /// Positive y is up and negative y is down. It should be interpreted in the context of the `placement` attribute if that is present.
  pub relative_y: Option<Tenths>,
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

/// The [Link] element serves as an outgoing simple XLink.
/// 
/// If a relative link is used within a document that is part of a compressed MusicXML file, the link is relative to the root folder of the zip file.
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
pub struct Link {
  /// Element-specific attributes
  pub attributes: LinkAttributes,
  /// Element-specific content
  pub content: (),
}
