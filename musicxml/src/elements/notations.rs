use super::{
  AccidentalMark, Arpeggiate, Articulations, Dynamics, Fermata, Footnote, Glissando, Level, NonArpeggiate, Ornaments,
  OtherNotation, Slide, Slur, Technical, Tied, Tuplet,
};
use crate::datatypes::{Id, YesNo};
use alloc::{string::String, vec::Vec};
use musicxml_internal::*;
use musicxml_macros::*;

/// Attributes pertaining to the [Notations] element.
#[derive(Debug, Default, PartialEq, Eq, AttributeDeserialize, AttributeSerialize)]
pub struct NotationsAttributes {
  /// Specifies an ID that is unique to the entire document.
  pub id: Option<Id>,
  /// Specifies whether or not to print an object. It is yes if not specified.
  pub print_object: Option<YesNo>,
}

/// The [NotationContentTypes] contains the various notations that can be applied to a note or chord.
#[derive(Debug, PartialEq, Eq)]
pub enum NotationContentTypes {
  /// The [Tied] element indicates that a tie begins or ends with this note.
  Tied(Tied),
  /// The [Slur] element indicates that a slur begins or ends with this note.
  Slur(Slur),
  /// The [Tuplet] element indicates that this note is part of a tuplet.
  Tuplet(Tuplet),
  /// The [Glissando] element indicates that a glissando line is to be drawn.
  Glissando(Glissando),
  /// The [Slide] element indicates that a slide element is to be drawn.
  Slide(Slide),
  /// The [Ornaments] element indicates that an ornament is to be performed.
  Ornaments(Ornaments),
  /// The [Technical] element indicates a technical instruction.
  Technical(Technical),
  /// The [Articulations] element indicates that an articulation is to be performed.
  Articulations(Articulations),
  /// The [Dynamics] element indicates a dynamic marking.
  Dynamics(Dynamics),
  /// The [Fermata] element indicates that a fermata is to be performed.
  Fermata(Fermata),
  /// The [Arpeggiate] element indicates that an arpeggiation is to be performed.
  Arpeggiate(Arpeggiate),
  /// The [NonArpeggiate] element indicates that a non-arpeggiation is to be performed.
  NonArpeggiate(NonArpeggiate),
  /// The [AccidentalMark] element indicates an accidental mark.
  AccidentalMark(AccidentalMark),
  /// The [OtherNotation] element is used to define any notations not yet in the MusicXML format.
  OtherNotation(OtherNotation),
}

/// Contents of the [Notations] element.
#[derive(Debug, Default, PartialEq, Eq)]
pub struct NotationsContents {
  /// The [Footnote] element is used to specify editorial information or analysis.
  pub footnote: Option<Footnote>,
  /// The [Level] element is used to specify editorial information or analysis.
  pub level: Option<Level>,
  /// The [NotationContentTypes] contains the various notations that can be applied to a note or chord.
  pub notations: Vec<NotationContentTypes>,
}

impl ContentDeserializer for NotationsContents {
  fn deserialize(elements: &[XmlElement]) -> Result<Self, String> {
    let mut notations = NotationsContents::default();
    for child in elements {
      match child.name.as_str() {
        "footnote" => notations.footnote = Some(Footnote::deserialize(child)?),
        "level" => notations.level = Some(Level::deserialize(child)?),
        "tied" => notations
          .notations
          .push(NotationContentTypes::Tied(Tied::deserialize(child)?)),
        "slur" => notations
          .notations
          .push(NotationContentTypes::Slur(Slur::deserialize(child)?)),
        "tuplet" => notations
          .notations
          .push(NotationContentTypes::Tuplet(Tuplet::deserialize(child)?)),
        "glissando" => notations
          .notations
          .push(NotationContentTypes::Glissando(Glissando::deserialize(child)?)),
        "slide" => notations
          .notations
          .push(NotationContentTypes::Slide(Slide::deserialize(child)?)),
        "ornaments" => notations
          .notations
          .push(NotationContentTypes::Ornaments(Ornaments::deserialize(child)?)),
        "technical" => notations
          .notations
          .push(NotationContentTypes::Technical(Technical::deserialize(child)?)),
        "articulations" => notations
          .notations
          .push(NotationContentTypes::Articulations(Articulations::deserialize(child)?)),
        "dynamics" => notations
          .notations
          .push(NotationContentTypes::Dynamics(Dynamics::deserialize(child)?)),
        "fermata" => notations
          .notations
          .push(NotationContentTypes::Fermata(Fermata::deserialize(child)?)),
        "arpeggiate" => notations
          .notations
          .push(NotationContentTypes::Arpeggiate(Arpeggiate::deserialize(child)?)),
        "non-arpeggiate" => notations
          .notations
          .push(NotationContentTypes::NonArpeggiate(NonArpeggiate::deserialize(child)?)),
        "accidental-mark" => {
          notations
            .notations
            .push(NotationContentTypes::AccidentalMark(AccidentalMark::deserialize(
              child,
            )?));
        }
        "other-notation" => notations
          .notations
          .push(NotationContentTypes::OtherNotation(OtherNotation::deserialize(child)?)),
        _ => Err(format!("Unknown NotationsContent: {:?}", child.name))?,
      }
    }
    Ok(notations)
  }
}

impl ContentSerializer for NotationsContents {
  fn serialize(element: &Self) -> Vec<XmlElement> {
    let mut elements: Vec<XmlElement> = Vec::new();
    if let Some(content) = &element.footnote {
      elements.push(Footnote::serialize(content));
    }
    if let Some(content) = &element.level {
      elements.push(Level::serialize(content));
    }
    for notation in &element.notations {
      match notation {
        NotationContentTypes::Tied(content) => elements.push(Tied::serialize(content)),
        NotationContentTypes::Slur(content) => elements.push(Slur::serialize(content)),
        NotationContentTypes::Tuplet(content) => elements.push(Tuplet::serialize(content)),
        NotationContentTypes::Glissando(content) => elements.push(Glissando::serialize(content)),
        NotationContentTypes::Slide(content) => elements.push(Slide::serialize(content)),
        NotationContentTypes::Ornaments(content) => elements.push(Ornaments::serialize(content)),
        NotationContentTypes::Technical(content) => elements.push(Technical::serialize(content)),
        NotationContentTypes::Articulations(content) => elements.push(Articulations::serialize(content)),
        NotationContentTypes::Dynamics(content) => elements.push(Dynamics::serialize(content)),
        NotationContentTypes::Fermata(content) => elements.push(Fermata::serialize(content)),
        NotationContentTypes::Arpeggiate(content) => elements.push(Arpeggiate::serialize(content)),
        NotationContentTypes::NonArpeggiate(content) => elements.push(NonArpeggiate::serialize(content)),
        NotationContentTypes::AccidentalMark(content) => elements.push(AccidentalMark::serialize(content)),
        NotationContentTypes::OtherNotation(content) => elements.push(OtherNotation::serialize(content)),
      }
    }
    elements
  }
}

///The [Notations] element collects musical notations that apply to a specific note or chord.
///
/// Multiple [Notations] elements are allowed in order to represent multiple editorial levels.
/// The `print_object` attribute allows [Notations] to represent details of performance technique, such as fingerings,
/// without having them appear in the score. This element is not related to the concept of XML notations.
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
pub struct Notations {
  /// Element-specific attributes
  pub attributes: NotationsAttributes,
  #[flatten]
  /// Element-specific content
  pub content: NotationsContents,
}

#[cfg(test)]
mod notations_tests {
  use crate::elements::*;
  use crate::parser::parse_from_xml_str;
  use alloc::string::String;

  #[test]
  fn deserialize_valid1() {
    let result = parse_from_xml_str::<Notations>(
      "<notations>
        <footnote>Footnote</footnote>
        <slide type=\"start\">Slide Text</slide>
        <dynamics>
          <f/>
          <f/>
        </dynamics>
      </notations>",
    );
    assert!(result.is_ok());
    assert_eq!(
      result.unwrap(),
      Notations {
        attributes: NotationsAttributes::default(),
        content: NotationsContents {
          footnote: Some(Footnote {
            attributes: FootnoteAttributes::default(),
            content: String::from("Footnote")
          }),
          level: None,
          notations: vec![
            NotationContentTypes::Slide(Slide {
              attributes: SlideAttributes {
                r#type: crate::datatypes::StartStop::Start,
                accelerate: None,
                beats: None,
                color: None,
                dash_length: None,
                default_x: None,
                default_y: None,
                first_beat: None,
                font_family: None,
                font_size: None,
                font_style: None,
                font_weight: None,
                id: None,
                last_beat: None,
                line_type: None,
                number: None,
                relative_x: None,
                relative_y: None,
                space_length: None,
              },
              content: String::from("Slide Text")
            }),
            NotationContentTypes::Dynamics(Dynamics {
              attributes: DynamicsAttributes::default(),
              content: vec![
                DynamicsType::F(F {
                  attributes: (),
                  content: ()
                }),
                DynamicsType::F(F {
                  attributes: (),
                  content: ()
                })
              ]
            }),
          ]
        }
      }
    );
  }
}
