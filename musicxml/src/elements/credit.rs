use super::{Bookmark, CreditImage, CreditSymbol, CreditType, CreditWords, Link};
use crate::datatypes::{Id, PositiveInteger};
use musicxml_internal::*;
use musicxml_macros::*;

/// Attributes pertaining to the [Credit] element.
#[derive(Debug, Default, PartialEq, Eq, AttributeDeserialize, AttributeSerialize)]
pub struct CreditAttributes {
  /// Specifies an ID that is unique to the entire document.
  pub id: Option<Id>,
  /// Specifies the page number where the [Credit] should appear.
  /// This is an integer value that starts with 1 for the first page. Its value is 1 if not specified.
  /// Since credits occur before the music, these page numbers do not refer to the page numbering specified
  /// by the [Print][super::Print] element's `page_number` attribute.
  pub page: Option<PositiveInteger>,
}

#[derive(Debug, PartialEq, Eq, ContentDeserialize, ContentSerialize)]
pub struct CreditImageContents {
  pub credit_image: CreditImage,
}

#[derive(Debug, Default, PartialEq, Eq)]
pub struct CreditTextSubcontents {
  pub link: Vec<Link>,
  pub bookmark: Vec<Bookmark>,
  pub credit_words: Option<CreditWords>,
  pub credit_symbol: Option<CreditSymbol>,
}

#[derive(Debug, Default, PartialEq, Eq)]
pub struct CreditTextContents {
  pub credit_words: Option<CreditWords>,
  pub credit_symbol: Option<CreditSymbol>,
  pub additional: Vec<CreditTextSubcontents>,
}

impl ContentDeserializer for CreditTextContents {
  fn deserialize(elements: &Vec<XmlElement>) -> Result<Self, String> {
    let mut subcontents: Option<CreditTextSubcontents> = None;
    let mut contents = CreditTextContents::default();
    for element in elements {
      match element.name.as_str() {
        "link" => match subcontents.as_mut() {
          Some(content) => content.link.push(Link::deserialize(element)?),
          _ => (),
        },
        "bookmark" => match subcontents.as_mut() {
          Some(content) => content.bookmark.push(Bookmark::deserialize(element)?),
          _ => (),
        },
        "credit-words" => {
          match subcontents {
            Some(mut content) => {
              content.credit_words = Some(CreditWords::deserialize(element)?);
              contents.additional.push(content);
            }
            _ => contents.credit_words = Some(CreditWords::deserialize(element)?),
          }
          subcontents = Some(CreditTextSubcontents::default());
        }
        "credit-symbol" => {
          match subcontents {
            Some(mut content) => {
              content.credit_symbol = Some(CreditSymbol::deserialize(element)?);
              contents.additional.push(content);
            }
            _ => contents.credit_symbol = Some(CreditSymbol::deserialize(element)?),
          }
          subcontents = Some(CreditTextSubcontents::default());
        }
        _ => (),
      }
    }
    Ok(contents)
  }
}

#[derive(Debug, PartialEq, Eq)]
pub enum CreditSubcontents {
  Image(CreditImageContents),
  Text(CreditTextContents),
}

#[derive(Debug, PartialEq, Eq)]
pub struct CreditContents {
  pub credit_type: Vec<CreditType>,
  pub link: Vec<Link>,
  pub bookmark: Vec<Bookmark>,
  pub credit: CreditSubcontents,
}

impl ContentDeserializer for CreditContents {
  fn deserialize(elements: &Vec<XmlElement>) -> Result<Self, String> {
    let mut contents = CreditContents {
      credit_type: Vec::new(),
      link: Vec::new(),
      bookmark: Vec::new(),
      credit: if let Some(_) = elements.iter().find(|&el| el.name == "credit-image") {
        CreditSubcontents::Image(CreditImageContents::deserialize(elements)?)
      } else {
        CreditSubcontents::Text(CreditTextContents::deserialize(elements)?)
      },
    };
    let mut image_or_words_found = false;
    for element in elements {
      match element.name.as_str() {
        "credit-type" => contents.credit_type.push(CreditType::deserialize(element)?),
        "link" => {
          if !image_or_words_found {
            contents.link.push(Link::deserialize(element)?)
          }
        }
        "bookmark" => {
          if !image_or_words_found {
            contents.bookmark.push(Bookmark::deserialize(element)?)
          }
        }
        _ => {
          image_or_words_found = true;
        }
      }
    }
    Ok(contents)
  }
}

impl ContentSerializer for CreditContents {
  fn serialize(element: &Self) -> Vec<XmlElement> {
    let mut elements: Vec<XmlElement> = Vec::new();
    for el in &element.credit_type {
      elements.push(CreditType::serialize(el));
    }
    for el in &element.link {
      elements.push(Link::serialize(el));
    }
    for el in &element.bookmark {
      elements.push(Bookmark::serialize(el));
    }
    match &element.credit {
      CreditSubcontents::Image(contents) => {
        elements.push(CreditImage::serialize(&contents.credit_image));
      }
      CreditSubcontents::Text(contents) => {
        if let Some(content) = &contents.credit_words {
          elements.push(CreditWords::serialize(content));
        }
        if let Some(content) = &contents.credit_symbol {
          elements.push(CreditSymbol::serialize(content));
        }
        for el in &contents.additional {
          for content in &el.link {
            elements.push(Link::serialize(content));
          }
          for content in &el.bookmark {
            elements.push(Bookmark::serialize(content));
          }
          if let Some(content) = &el.credit_words {
            elements.push(CreditWords::serialize(content));
          }
          if let Some(content) = &el.credit_symbol {
            elements.push(CreditSymbol::serialize(content));
          }
        }
      }
    }
    elements
  }
}

/// The [Credit] element represents the appearance of the title, composer, arranger, lyricist, copyright, dedication, and other text, symbols, and graphics
/// that commonly appear on the first page of a score.
///
/// The [CreditWords][super::CreditWords], [CreditSymbol][super::CreditSymbol], and [CreditImage][super::CreditImage] elements are similar to the
/// [Words][super::Words], [Symbol][super::Symbol], and [Image][super::Image] elements for a [Direction][super::Direction]. However, since the credit is not
/// part of a measure, the `default_x` and `default_y` attributes adjust the origin relative to the bottom left-hand corner of the page. The `enclosure`
/// for [CreditWords][super::CreditWords] and [CreditSymbol][super::CreditSymbol] is none if not specified.
///
/// By default, a series of [CreditWords][super::CreditWords] and [CreditSymbol][super::CreditSymbol] elements within a single [Credit] element follow one another
/// in sequence visually. Non-positional formatting attributes are carried over from the previous element by default.
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
pub struct Credit {
  /// Element-specific attributes
  pub attributes: CreditAttributes,
  #[flatten]
  /// Element-specific content
  pub content: CreditContents,
}

#[cfg(test)]
mod credit_tests {
  use super::*;
  use crate::datatypes::{AnyUri, EnclosureShape, Id, LeftCenterRight, NumberOrNormal, SmuflGlyphName, TextDirection};
  use crate::elements::{
    BookmarkAttributes, CreditImageAttributes, CreditSymbolAttributes, CreditWordsAttributes, LinkAttributes,
  };
  use crate::parser::parse_from_xml_str;

  #[test]
  fn test1() {
    let result = parse_from_xml_str::<Credit>(
      "<credit>
        <credit-type>Title</credit-type>
        <credit-image halign=\"right\"/>
      </credit>",
    );
    assert_eq!(
      result.unwrap(),
      Credit {
        attributes: CreditAttributes { ..Default::default() },
        content: CreditContents {
          credit_type: vec![CreditType {
            attributes: (),
            content: String::from("Title")
          }],
          link: vec![],
          bookmark: vec![],
          credit: CreditSubcontents::Image(CreditImageContents {
            credit_image: CreditImage {
              attributes: CreditImageAttributes {
                halign: Some(LeftCenterRight::Right),
                ..Default::default()
              },
              content: ()
            }
          })
        },
      }
    );
  }

  #[test]
  fn test2() {
    let result = parse_from_xml_str::<Credit>(
      "<credit>
        <credit-type>Artist</credit-type>
        <link xlink:href=\"root/Link\"/>
        <credit-words dir=\"rtl\">Credit Text</credit-words>
        <credit-words enclosure=\"inverted-bracket\">Credit Text2</credit-words>
        <link xlink:href=\"root/Link2\"/>
        <bookmark id=\"BookmarkID\"/>
        <credit-symbol line-height=\"13\">Symbol</credit-symbol>
      </credit>",
    );
    assert_eq!(
      result.unwrap(),
      Credit {
        attributes: CreditAttributes { ..Default::default() },
        content: CreditContents {
          credit_type: vec![CreditType {
            attributes: (),
            content: String::from("Artist")
          }],
          link: vec![Link {
            attributes: LinkAttributes {
              xlink_href: AnyUri(String::from("root/Link")),
              default_x: None,
              default_y: None,
              element: None,
              name: None,
              position: None,
              relative_x: None,
              relative_y: None,
              xlink_actuate: None,
              xlink_role: None,
              xlink_show: None,
              xlink_title: None,
              xlink_type: None
            },
            content: ()
          }],
          bookmark: vec![],
          credit: CreditSubcontents::Text(CreditTextContents {
            credit_words: Some(CreditWords {
              attributes: CreditWordsAttributes {
                dir: Some(TextDirection::Rtl),
                ..Default::default()
              },
              content: String::from("Credit Text")
            }),
            credit_symbol: None,
            additional: vec![
              CreditTextSubcontents {
                link: Vec::new(),
                bookmark: Vec::new(),
                credit_words: Some(CreditWords {
                  attributes: CreditWordsAttributes {
                    enclosure: Some(EnclosureShape::InvertedBracket),
                    ..Default::default()
                  },
                  content: String::from("Credit Text2")
                }),
                credit_symbol: None,
              },
              CreditTextSubcontents {
                link: vec![Link {
                  attributes: LinkAttributes {
                    xlink_href: AnyUri(String::from("root/Link2")),
                    default_x: None,
                    default_y: None,
                    element: None,
                    name: None,
                    position: None,
                    relative_x: None,
                    relative_y: None,
                    xlink_actuate: None,
                    xlink_role: None,
                    xlink_show: None,
                    xlink_title: None,
                    xlink_type: None
                  },
                  content: ()
                }],
                bookmark: vec![Bookmark {
                  attributes: BookmarkAttributes {
                    id: Id(String::from("BookmarkID")),
                    element: None,
                    name: None,
                    position: None
                  },
                  content: ()
                }],
                credit_words: None,
                credit_symbol: Some(CreditSymbol {
                  attributes: CreditSymbolAttributes {
                    line_height: Some(NumberOrNormal::Decimal(13.0)),
                    ..Default::default()
                  },
                  content: SmuflGlyphName(String::from("Symbol"))
                })
              }
            ]
          })
        },
      }
    );
  }
}
