use crate::datatypes::{Id, NmToken, PositiveInteger, Token};
use musicxml_internal::*;
use musicxml_macros::*;

/// Attributes pertaining to the [Bookmark] element.
#[derive(Debug, PartialEq, Eq, AttributeDeserialize, AttributeSerialize)]
pub struct BookmarkAttributes {
  /// The identifier for this bookmark, unique within this document.
  pub id: Id,
  /// The `element` attribute specifies an element type for a descendant of the next sibling element
  /// that is not a [Link][super::Link] or [Bookmark] element. When not present, the [Bookmark] or
  /// [Link][super::Link] element refers to the next sibling element in the MusicXML file.
  pub element: Option<NmToken>,
  /// The name for this bookmark.
  pub name: Option<Token>,
  /// The `position` attribute specifies the position of the descendant element specified by the `element` attribute,
  /// where the first position is 1. The `position` attribute is ignored if the `element` attribute is not present.
  ///
  /// For instance, an `element` value of "beam" and a `position` value of "2" defines the [Link][super::Link] or
  /// [Bookmark] to refer to the second beam descendant of the next sibling element that is not a [Link][super::Link]
  /// or [Bookmark] element. This is equivalent to an XPath test of `[.//beam[2]]` done in the context of the sibling element.
  pub position: Option<PositiveInteger>,
}

/// The [Bookmark] element serves as a well-defined target for an incoming simple XLink.
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
pub struct Bookmark {
  /// Element-specific attributes
  pub attributes: BookmarkAttributes,
  /// Element-specific content
  pub content: (),
}
