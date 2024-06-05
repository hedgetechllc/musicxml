use super::{Elision, EndLine, EndParagraph, Extend, Footnote, Humming, Laughing, Level, Syllabic, Text};
use crate::datatypes::{AboveBelow, Color, Id, LeftCenterRight, NmToken, Tenths, TimeOnly, Token, YesNo};
use musicxml_internal::*;
use musicxml_macros::*;

/// Attributes pertaining to the [Lyric] element.
#[derive(Debug, Default, PartialEq, Eq, AttributeDeserialize, AttributeSerialize)]
pub struct LyricAttributes {
  /// Indicates the color of an element.
  pub color: Option<Color>,
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
  /// Specifies an ID that is unique to the entire document.
  pub id: Option<Id>,
  /// Indicates left, center, or right justification. The default value varies for different elements.
  /// For elements where the `justify` attribute is present but the `halign` attribute is not,
  /// the `justify` attribute indicates horizontal alignment as well as justification.
  pub justify: Option<LeftCenterRight>,
  /// Indicates the name of the lyric type. Common examples are verse and chorus.
  pub name: Option<Token>,
  /// Specifies the lyric line when multiple lines are present.
  pub number: Option<NmToken>,
  /// Indicates whether something is above or below another element, such as a note or a notation.
  pub placement: Option<AboveBelow>,
  /// Specifies whether or not to print an object. It is yes if not specified.
  pub print_object: Option<YesNo>,
  /// Changes the horizontal position relative to the default position, either as computed by the individual program, or as overridden by the `default_x` attribute.
  /// Positive x is right and negative x is left. It should be interpreted in the context of the [Offset][super::Offset] element or directive attribute if those are present.
  pub relative_x: Option<Tenths>,
  /// Changes the vertical position relative to the default position, either as computed by the individual program, or as overridden by the `default_y` attribute.
  /// Positive y is up and negative y is down. It should be interpreted in the context of the `placement` attribute if that is present.
  pub relative_y: Option<Tenths>,
  /// Restricts the type to apply to a set of times through a repeated section.
  /// If missing, the type applies all times through the repeated section.
  pub time_only: Option<TimeOnly>,
}

/// Contents of the [AdditionalTextLyric] element.
#[derive(Debug, PartialEq, Eq)]
pub struct AdditionalTextLyric {
  /// The [Elision] element represents an elision in a lyric.
  pub elision: Option<Elision>,
  /// The [Syllabic] element represents the type of syllable for a lyric.
  pub syllabic: Option<Syllabic>,
  /// The [Text] element represents the text of a lyric.
  pub text: Text,
}

/// The [TextLyric] element represents the text of a lyric.
#[derive(Debug, Default, PartialEq, Eq)]
pub struct TextLyric {
  /// The [Syllabic] element represents the type of syllable for a lyric.
  pub syllabic: Option<Syllabic>,
  /// The [Text] element represents the text of a lyric.
  pub text: Text,
  /// Additional text elements that are part of the same syllable.
  pub additional: Vec<AdditionalTextLyric>,
  /// The [Extend] element represents an extension of a lyric.
  pub extend: Option<Extend>,
  /// The [EndLine] element specifies the end of a line of lyrics.
  pub end_line: Option<EndLine>,
  /// The [EndParagraph] element specifies the end of a paragraph of lyrics.
  pub end_paragraph: Option<EndParagraph>,
  /// The [Footnote] element specifies editorial information or lyrics content.
  pub footnote: Option<Footnote>,
  /// The [Level] element specifies the editorial level of a score or part.
  pub level: Option<Level>,
}

impl ContentDeserializer for TextLyric {
  fn deserialize(elements: &Vec<XmlElement>) -> Result<Self, String> {
    let mut text_lyric = TextLyric::default();
    for el in elements {
      match el.name.as_str() {
        "syllabic" => {
          if text_lyric.syllabic.is_none() {
            text_lyric.syllabic = Some(Syllabic::deserialize(el)?)
          } else if !text_lyric.additional.is_empty() {
            text_lyric.additional.last_mut().unwrap().syllabic = Some(Syllabic::deserialize(el)?)
          }
        }
        "text" => {
          if text_lyric.text.content.is_empty() {
            text_lyric.text = Text::deserialize(el)?
          } else if text_lyric.additional.is_empty() {
            text_lyric.additional.push(AdditionalTextLyric {
              elision: None,
              syllabic: None,
              text: Text::deserialize(el)?,
            })
          } else if text_lyric.additional.last().unwrap().text.content.is_empty() {
            text_lyric.additional.last_mut().unwrap().text = Text::deserialize(el)?
          } else {
            text_lyric.additional.push(AdditionalTextLyric {
              elision: None,
              syllabic: None,
              text: Text::deserialize(el)?,
            })
          }
        }
        "elision" => text_lyric.additional.push(AdditionalTextLyric {
          elision: Some(Elision::deserialize(el)?),
          syllabic: None,
          text: Text::default(),
        }),
        "extend" => text_lyric.extend = Some(Extend::deserialize(el)?),
        "end-line" => text_lyric.end_line = Some(EndLine::deserialize(el)?),
        "end-paragraph" => text_lyric.end_paragraph = Some(EndParagraph::deserialize(el)?),
        "footnote" => text_lyric.footnote = Some(Footnote::deserialize(el)?),
        "level" => text_lyric.level = Some(Level::deserialize(el)?),
        _ => {}
      }
    }
    Ok(text_lyric)
  }
}

impl ContentSerializer for TextLyric {
  fn serialize(element: &Self) -> Vec<XmlElement> {
    let mut elements: Vec<XmlElement> = Vec::new();
    if let Some(content) = &element.syllabic {
      elements.push(Syllabic::serialize(content));
    }
    elements.push(Text::serialize(&element.text));
    for el in &element.additional {
      if let Some(content) = &el.elision {
        elements.push(Elision::serialize(content));
      }
      if let Some(content) = &el.syllabic {
        elements.push(Syllabic::serialize(content));
      }
      elements.push(Text::serialize(&el.text));
    }
    if let Some(content) = &element.extend {
      elements.push(Extend::serialize(content));
    }
    if let Some(content) = &element.end_line {
      elements.push(EndLine::serialize(content));
    }
    if let Some(content) = &element.end_paragraph {
      elements.push(EndParagraph::serialize(content));
    }
    if let Some(content) = &element.footnote {
      elements.push(Footnote::serialize(content));
    }
    if let Some(content) = &element.level {
      elements.push(Level::serialize(content));
    }
    elements
  }
}

/// The [ExtendLyric] element represents an extension of a lyric.
#[derive(Debug, PartialEq, Eq, ContentDeserialize, ContentSerialize)]
pub struct ExtendLyric {
  /// The [Extend] element represents an extension of a lyric.
  pub extend: Extend,
  /// The [EndLine] element specifies the end of a line of lyrics.
  pub end_line: Option<EndLine>,
  /// The [EndParagraph] element specifies the end of a paragraph of lyrics.
  pub end_paragraph: Option<EndParagraph>,
  /// The [Footnote] element specifies editorial information or lyrics content.
  pub footnote: Option<Footnote>,
  /// The [Level] element specifies the editorial level of a score or part.
  pub level: Option<Level>,
}

/// The [LaughingLyric] element represents a laughing lyric.
#[derive(Debug, PartialEq, Eq, ContentDeserialize, ContentSerialize)]
pub struct LaughingLyric {
  /// The [Laughing] element represents a laughing lyric.
  pub laughing: Laughing,
  /// The [EndLine] element specifies the end of a line of lyrics.
  pub end_line: Option<EndLine>,
  /// The [EndParagraph] element specifies the end of a paragraph of lyrics.
  pub end_paragraph: Option<EndParagraph>,
  /// The [Footnote] element specifies editorial information or lyrics content.
  pub footnote: Option<Footnote>,
  /// The [Level] element specifies the editorial level of a score or part.
  pub level: Option<Level>,
}

/// The [HummingLyric] element represents a humming lyric.
#[derive(Debug, PartialEq, Eq, ContentDeserialize, ContentSerialize)]
pub struct HummingLyric {
  /// The [Humming] element represents a humming lyric.
  pub humming: Humming,
  /// The [EndLine] element specifies the end of a line of lyrics.
  pub end_line: Option<EndLine>,
  /// The [EndParagraph] element specifies the end of a paragraph of lyrics.
  pub end_paragraph: Option<EndParagraph>,
  /// The [Footnote] element specifies editorial information or lyrics content.
  pub footnote: Option<Footnote>,
  /// The [Level] element specifies the editorial level of a score or part.
  pub level: Option<Level>,
}

/// Contents of the [Lyric] element.
///
/// The [Lyric] element may contain either a [TextLyric], [ExtendLyric], [LaughingLyric], or [HummingLyric].
#[derive(Debug, PartialEq, Eq)]
pub enum LyricContents {
  /// The [TextLyric] element represents the text of a lyric.
  Text(TextLyric),
  /// The [ExtendLyric] element represents an extension of a lyric.
  Extend(ExtendLyric),
  /// The [LaughingLyric] element represents a laughing lyric.
  Laughing(LaughingLyric),
  /// The [HummingLyric] element represents a humming lyric.
  Humming(HummingLyric),
}

impl ContentDeserializer for LyricContents {
  fn deserialize(elements: &Vec<XmlElement>) -> Result<Self, String> {
    Ok(if let Some(_) = elements.iter().find(|&el| el.name == "text") {
      LyricContents::Text(TextLyric::deserialize(elements)?)
    } else if let Some(_) = elements.iter().find(|&el| el.name == "laughing") {
      LyricContents::Laughing(LaughingLyric::deserialize(elements)?)
    } else if let Some(_) = elements.iter().find(|&el| el.name == "humming") {
      LyricContents::Humming(HummingLyric::deserialize(elements)?)
    } else {
      LyricContents::Extend(ExtendLyric::deserialize(elements)?)
    })
  }
}

impl ContentSerializer for LyricContents {
  fn serialize(element: &Self) -> Vec<XmlElement> {
    match &element {
      LyricContents::Text(content) => TextLyric::serialize(content),
      LyricContents::Extend(content) => ExtendLyric::serialize(content),
      LyricContents::Laughing(content) => LaughingLyric::serialize(content),
      LyricContents::Humming(content) => HummingLyric::serialize(content),
    }
  }
}

/// The [Lyric] element represents text underlays for lyrics.
///
/// ![Lyric](https://hedgetechllc.github.io/musicxml/musicxml/elements/lyric.png)
///
/// Two [Text] elements that are not separated by an [Elision] element are part of the same syllable, but may have different text formatting.
/// A second [Syllabic] element is not allowed unless preceded by an [Elision] element.
///
/// If not otherwise specified:
///
/// - The `justify` value is center.
/// - The `placement` value is below.
/// - The `valign` value is baseline.
/// - The `halign` value matches the `justify` value.
///
/// The `print_object` attribute can override a [Note][super::Note]'s `print_lyric` attribute in cases where only some lyrics on a note are printed,
/// as when lyrics for later verses are printed in a block of text rather than with each note.
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
pub struct Lyric {
  /// Element-specific attributes
  pub attributes: LyricAttributes,
  #[flatten]
  /// Element-specific content
  pub content: LyricContents,
}

#[cfg(test)]
mod lyric_tests {
  use super::*;
  use crate::elements::*;
  use crate::parser::parse_from_xml_str;

  #[test]
  fn deserialize_valid1() {
    let result = parse_from_xml_str::<Lyric>(
      "<lyric time-only=\"1,3\">
        <syllabic>middle</syllabic>
        <text>hello</text>
        <elision>start</elision>
        <syllabic>end</syllabic>
        <text>world</text>
        <elision>stop</elision>
        <text>next</text>
        <text>final</text>
        <extend/>
        <end-line/>
      </lyric>",
    );
    assert!(result.is_ok());
    assert_eq!(
      result.unwrap(),
      Lyric {
        attributes: LyricAttributes {
          time_only: Some(TimeOnly(vec![1, 3])),
          ..Default::default()
        },
        content: LyricContents::Text(TextLyric {
          syllabic: Some(Syllabic {
            attributes: (),
            content: crate::datatypes::Syllabic::Middle
          }),
          text: Text {
            attributes: TextAttributes::default(),
            content: String::from("hello")
          },
          additional: vec![
            AdditionalTextLyric {
              elision: Some(Elision {
                attributes: ElisionAttributes::default(),
                content: String::from("start")
              }),
              syllabic: Some(Syllabic {
                attributes: (),
                content: crate::datatypes::Syllabic::End
              }),
              text: Text {
                attributes: TextAttributes::default(),
                content: String::from("world")
              },
            },
            AdditionalTextLyric {
              elision: Some(Elision {
                attributes: ElisionAttributes::default(),
                content: String::from("stop")
              }),
              syllabic: None,
              text: Text {
                attributes: TextAttributes::default(),
                content: String::from("next")
              },
            },
            AdditionalTextLyric {
              elision: None,
              syllabic: None,
              text: Text {
                attributes: TextAttributes::default(),
                content: String::from("final")
              },
            },
          ],
          extend: Some(Extend {
            attributes: ExtendAttributes::default(),
            content: ()
          }),
          end_line: Some(EndLine {
            attributes: (),
            content: ()
          }),
          end_paragraph: None,
          footnote: None,
          level: None,
        }),
      }
    );
  }

  #[test]
  fn deserialize_valid2() {
    let result = parse_from_xml_str::<Lyric>(
      "<lyric>
        <extend/>
        <end-paragraph/>
      </lyric>",
    );
    assert!(result.is_ok());
    assert_eq!(
      result.unwrap(),
      Lyric {
        attributes: LyricAttributes::default(),
        content: LyricContents::Extend(ExtendLyric {
          extend: Extend {
            attributes: ExtendAttributes::default(),
            content: ()
          },
          end_line: None,
          end_paragraph: Some(EndParagraph {
            attributes: (),
            content: ()
          }),
          footnote: None,
          level: None,
        }),
      }
    );
  }

  #[test]
  fn deserialize_valid3() {
    let result = parse_from_xml_str::<Lyric>(
      "<lyric>
        <laughing/>
      </lyric>",
    );
    assert!(result.is_ok());
    assert_eq!(
      result.unwrap(),
      Lyric {
        attributes: LyricAttributes::default(),
        content: LyricContents::Laughing(LaughingLyric {
          laughing: Laughing {
            attributes: (),
            content: ()
          },
          end_line: None,
          end_paragraph: None,
          footnote: None,
          level: None,
        }),
      }
    );
  }

  #[test]
  fn deserialize_valid4() {
    let result = parse_from_xml_str::<Lyric>(
      "<lyric>
        <humming/>
      </lyric>",
    );
    assert!(result.is_ok());
    assert_eq!(
      result.unwrap(),
      Lyric {
        attributes: LyricAttributes::default(),
        content: LyricContents::Humming(HummingLyric {
          humming: Humming {
            attributes: (),
            content: ()
          },
          end_line: None,
          end_paragraph: None,
          footnote: None,
          level: None,
        }),
      }
    );
  }
}
