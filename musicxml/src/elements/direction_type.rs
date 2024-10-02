use super::{
  AccordionRegistration, Bracket, Coda, Damp, DampAll, Dashes, Dynamics, Eyeglasses, HarpPedals, Image, Metronome,
  OctaveShift, OtherDirection, Pedal, Percussion, PrincipalVoice, Rehearsal, Scordatura, Segno, StaffDivide,
  StringMute, Symbol, Wedge, Words,
};
use crate::datatypes::Id;
use alloc::{string::String, vec::Vec};
use musicxml_internal::*;
use musicxml_macros::*;

/// Attributes pertaining to the [DirectionType] element.
#[derive(Debug, Default, PartialEq, Eq, AttributeDeserialize, AttributeSerialize)]
pub struct DirectionTypeAttributes {
  /// Specifies an ID that is unique to the entire document.
  pub id: Option<Id>,
}

/// The [DirectionTypeContents] element specifies all possible options available for use in a [DirectionType] element.
#[derive(Debug, PartialEq, Eq)]
pub enum DirectionTypeContents {
  /// The [Rehearsal] element indicates a rehearsal mark.
  Rehearsal(Vec<Rehearsal>),
  /// The [Segno] element indicates a segno mark.
  Segno(Vec<Segno>),
  /// The [Coda] element indicates a coda mark.
  Coda(Vec<Coda>),
  /// The [Words] element specifies a text direction.
  Words(Vec<Words>),
  /// The [Symbol] element specifies a musical symbol.
  Symbol(Vec<Symbol>),
  /// The [Wedge] element specifies a wedge symbol.
  Wedge(Wedge),
  /// The [Dynamics] element specifies a dynamic mark.
  Dynamics(Vec<Dynamics>),
  /// The [Dashes] element specifies a dashed line.
  Dashes(Dashes),
  /// The [Bracket] element specifies a bracket.
  Bracket(Bracket),
  /// The [Pedal] element specifies a piano pedal mark.
  Pedal(Pedal),
  /// The [Metronome] element specifies a metronome mark.
  Metronome(Metronome),
  /// The [OctaveShift] element specifies an octave shift.
  OctaveShift(OctaveShift),
  /// The [HarpPedals] element specifies harp pedal settings.
  HarpPedals(HarpPedals),
  /// The [Damp] element specifies a damp mark.
  Damp(Damp),
  /// The [DampAll] element specifies a damp all mark.
  DampAll(DampAll),
  /// The [Eyeglasses] element specifies eyeglasses.
  Eyeglasses(Eyeglasses),
  /// The [StringMute] element specifies a string mute.
  StringMute(StringMute),
  /// The [Scordatura] element specifies scordatura tuning.
  Scordatura(Scordatura),
  /// The [Image] element specifies an image.
  Image(Image),
  /// The [PrincipalVoice] element specifies the principal voice.
  PrincipalVoice(PrincipalVoice),
  /// The [Percussion] element specifies percussion notation.
  Percussion(Vec<Percussion>),
  /// The [AccordionRegistration] element specifies accordion registration.
  AccordionRegistration(AccordionRegistration),
  /// The [StaffDivide] element specifies a staff divide.
  StaffDivide(StaffDivide),
  /// The [OtherDirection] element specifies a direction not yet defined.
  OtherDirection(OtherDirection),
}

impl ContentDeserializer for DirectionTypeContents {
  fn deserialize(elements: &[XmlElement]) -> Result<Self, String> {
    Ok(if let Some(element) = elements.first() {
      match element.name.as_str() {
        "rehearsal" => DirectionTypeContents::Rehearsal(
          elements
            .iter()
            .filter_map(|el| {
              if el.name == "rehearsal" {
                Rehearsal::deserialize(el).ok()
              } else {
                None
              }
            })
            .collect::<Vec<_>>(),
        ),
        "segno" => DirectionTypeContents::Segno(
          elements
            .iter()
            .filter_map(|el| {
              if el.name == "segno" {
                Segno::deserialize(el).ok()
              } else {
                None
              }
            })
            .collect::<Vec<_>>(),
        ),
        "coda" => DirectionTypeContents::Coda(
          elements
            .iter()
            .filter_map(|el| {
              if el.name == "coda" {
                Coda::deserialize(el).ok()
              } else {
                None
              }
            })
            .collect::<Vec<_>>(),
        ),
        "words" => DirectionTypeContents::Words(
          elements
            .iter()
            .filter_map(|el| {
              if el.name == "words" {
                Words::deserialize(el).ok()
              } else {
                None
              }
            })
            .collect::<Vec<_>>(),
        ),
        "symbol" => DirectionTypeContents::Symbol(
          elements
            .iter()
            .filter_map(|el| {
              if el.name == "symbol" {
                Symbol::deserialize(el).ok()
              } else {
                None
              }
            })
            .collect::<Vec<_>>(),
        ),
        "wedge" => DirectionTypeContents::Wedge(Wedge::deserialize(element)?),
        "dynamics" => DirectionTypeContents::Dynamics(
          elements
            .iter()
            .filter_map(|el| {
              if el.name == "dynamics" {
                Dynamics::deserialize(el).ok()
              } else {
                None
              }
            })
            .collect::<Vec<_>>(),
        ),
        "dashes" => DirectionTypeContents::Dashes(Dashes::deserialize(element)?),
        "bracket" => DirectionTypeContents::Bracket(Bracket::deserialize(element)?),
        "pedal" => DirectionTypeContents::Pedal(Pedal::deserialize(element)?),
        "metronome" => DirectionTypeContents::Metronome(Metronome::deserialize(element)?),
        "octave-shift" => DirectionTypeContents::OctaveShift(OctaveShift::deserialize(element)?),
        "harp-pedals" => DirectionTypeContents::HarpPedals(HarpPedals::deserialize(element)?),
        "damp" => DirectionTypeContents::Damp(Damp::deserialize(element)?),
        "damp-all" => DirectionTypeContents::DampAll(DampAll::deserialize(element)?),
        "eyeglasses" => DirectionTypeContents::Eyeglasses(Eyeglasses::deserialize(element)?),
        "string-mute" => DirectionTypeContents::StringMute(StringMute::deserialize(element)?),
        "scordatura" => DirectionTypeContents::Scordatura(Scordatura::deserialize(element)?),
        "image" => DirectionTypeContents::Image(Image::deserialize(element)?),
        "principal-voice" => DirectionTypeContents::PrincipalVoice(PrincipalVoice::deserialize(element)?),
        "percussion" => DirectionTypeContents::Percussion(
          elements
            .iter()
            .filter_map(|el| {
              if el.name == "percussion" {
                Percussion::deserialize(el).ok()
              } else {
                None
              }
            })
            .collect::<Vec<_>>(),
        ),
        "accordion-registration" => {
          DirectionTypeContents::AccordionRegistration(AccordionRegistration::deserialize(element)?)
        }
        "staff-divide" => DirectionTypeContents::StaffDivide(StaffDivide::deserialize(element)?),
        "other-direction" => DirectionTypeContents::OtherDirection(OtherDirection::deserialize(element)?),
        other => return Err(format!("Unknown child element <{}> of <direction-type>", other)),
      }
    } else {
      Err(format!("Missing required child of element <direction-type>"))?
    })
  }
}

impl ContentSerializer for DirectionTypeContents {
  fn serialize(element: &Self) -> Vec<XmlElement> {
    let mut elements: Vec<XmlElement> = Vec::new();
    match element {
      DirectionTypeContents::Rehearsal(contents) => {
        for element in contents {
          elements.push(Rehearsal::serialize(element));
        }
      }
      DirectionTypeContents::Segno(contents) => {
        for element in contents {
          elements.push(Segno::serialize(element));
        }
      }
      DirectionTypeContents::Coda(contents) => {
        for element in contents {
          elements.push(Coda::serialize(element));
        }
      }
      DirectionTypeContents::Words(contents) => {
        for element in contents {
          elements.push(Words::serialize(element));
        }
      }
      DirectionTypeContents::Symbol(contents) => {
        for element in contents {
          elements.push(Symbol::serialize(element));
        }
      }
      DirectionTypeContents::Wedge(contents) => elements.push(Wedge::serialize(contents)),
      DirectionTypeContents::Dynamics(contents) => {
        for element in contents {
          elements.push(Dynamics::serialize(element));
        }
      }
      DirectionTypeContents::Dashes(contents) => elements.push(Dashes::serialize(contents)),
      DirectionTypeContents::Bracket(contents) => elements.push(Bracket::serialize(contents)),
      DirectionTypeContents::Pedal(contents) => elements.push(Pedal::serialize(contents)),
      DirectionTypeContents::Metronome(contents) => elements.push(Metronome::serialize(contents)),
      DirectionTypeContents::OctaveShift(contents) => elements.push(OctaveShift::serialize(contents)),
      DirectionTypeContents::HarpPedals(contents) => elements.push(HarpPedals::serialize(contents)),
      DirectionTypeContents::Damp(contents) => elements.push(Damp::serialize(contents)),
      DirectionTypeContents::DampAll(contents) => elements.push(DampAll::serialize(contents)),
      DirectionTypeContents::Eyeglasses(contents) => elements.push(Eyeglasses::serialize(contents)),
      DirectionTypeContents::StringMute(contents) => elements.push(StringMute::serialize(contents)),
      DirectionTypeContents::Scordatura(contents) => elements.push(Scordatura::serialize(contents)),
      DirectionTypeContents::Image(contents) => elements.push(Image::serialize(contents)),
      DirectionTypeContents::PrincipalVoice(contents) => elements.push(PrincipalVoice::serialize(contents)),
      DirectionTypeContents::Percussion(contents) => {
        for element in contents {
          elements.push(Percussion::serialize(element));
        }
      }
      DirectionTypeContents::AccordionRegistration(contents) => {
        elements.push(AccordionRegistration::serialize(contents))
      }
      DirectionTypeContents::StaffDivide(contents) => elements.push(StaffDivide::serialize(contents)),
      DirectionTypeContents::OtherDirection(contents) => elements.push(OtherDirection::serialize(contents)),
    }
    elements
  }
}

/// Textual direction types may have more than 1 component due to multiple fonts.
///
/// The [Dynamics] element may also be used in the [Notations][super::Notations] element. Child element attributes related to print suggestions apply
/// to the individual [DirectionType], not to the overall [Direction][super::Direction].
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
#[rename("direction-type")]
pub struct DirectionType {
  /// Element-specific attributes
  pub attributes: DirectionTypeAttributes,
  #[flatten]
  /// Element-specific content
  pub content: DirectionTypeContents,
}

#[cfg(test)]
mod direction_type_tests {
  use super::*;
  use crate::datatypes::{BeamValue, NonNegativeInteger, NoteTypeValue, StartStop};
  use crate::elements::{
    ActualNotes, AdditionalMetronomeBasedContents, MetronomeAttributes, MetronomeBased, MetronomeBeam, NormalNotes,
  };
  use crate::elements::{
    MetronomeBeamAttributes, MetronomeTied, MetronomeTiedAttributes, MetronomeTuplet, MetronomeTupletAttributes,
    MetronomeTupletContents,
  };
  use crate::elements::{
    MetronomeContents, MetronomeDot, MetronomeNote, MetronomeNoteContents, MetronomeRelation, MetronomeType,
    RehearsalAttributes,
  };
  use crate::parser::parse_from_xml_str;

  #[test]
  fn deserialize_valid1() {
    let result = parse_from_xml_str::<DirectionType>(
      "
      <direction-type id=\"Test ID\">
        <rehearsal>1</rehearsal>
        <rehearsal>2</rehearsal>
        <rehearsal>3</rehearsal>
      </direction-type>",
    );
    assert!(result.is_ok());
    assert_eq!(
      result.unwrap(),
      DirectionType {
        attributes: DirectionTypeAttributes {
          id: Some(Id(String::from("Test ID"))),
        },
        content: DirectionTypeContents::Rehearsal(vec![
          Rehearsal {
            attributes: RehearsalAttributes::default(),
            content: String::from("1")
          },
          Rehearsal {
            attributes: RehearsalAttributes::default(),
            content: String::from("2")
          },
          Rehearsal {
            attributes: RehearsalAttributes::default(),
            content: String::from("3")
          },
        ])
      }
    );
  }

  #[test]
  fn deserialize_valid2() {
    let result = parse_from_xml_str::<DirectionType>(
      "
      <direction-type>
        <metronome>
          <metronome-note>
            <metronome-type>quarter</metronome-type>
            <metronome-dot/>
            <metronome-dot/>
            <metronome-tied type=\"start\"/>
          </metronome-note>
          <metronome-note>
            <metronome-type>eighth</metronome-type>
            <metronome-beam>begin</metronome-beam>
            <metronome-beam>continue</metronome-beam>
          </metronome-note>
          <metronome-relation>Relation String</metronome-relation>
          <metronome-note>
            <metronome-type>1024th</metronome-type>
            <metronome-tuplet type=\"stop\">
              <actual-notes>3</actual-notes>
              <normal-notes>2</normal-notes>
            </metronome-tuplet>
          </metronome-note>
        </metronome>
      </direction-type>",
    );
    assert!(result.is_ok());
    assert_eq!(
      result.unwrap(),
      DirectionType {
        attributes: DirectionTypeAttributes::default(),
        content: DirectionTypeContents::Metronome(Metronome {
          attributes: MetronomeAttributes::default(),
          content: MetronomeContents::MetronomeBased(MetronomeBased {
            metronome_arrows: None,
            metronome_note: vec![
              MetronomeNote {
                attributes: (),
                content: MetronomeNoteContents {
                  metronome_type: MetronomeType {
                    attributes: (),
                    content: NoteTypeValue::Quarter
                  },
                  metronome_dot: vec![MetronomeDot::default(), MetronomeDot::default()],
                  metronome_beam: vec![],
                  metronome_tied: Some(MetronomeTied {
                    attributes: MetronomeTiedAttributes {
                      r#type: StartStop::Start
                    },
                    content: ()
                  }),
                  metronome_tuplet: None
                }
              },
              MetronomeNote {
                attributes: (),
                content: MetronomeNoteContents {
                  metronome_type: MetronomeType {
                    attributes: (),
                    content: NoteTypeValue::Eighth
                  },
                  metronome_dot: vec![],
                  metronome_beam: vec![
                    MetronomeBeam {
                      attributes: MetronomeBeamAttributes::default(),
                      content: BeamValue::Begin
                    },
                    MetronomeBeam {
                      attributes: MetronomeBeamAttributes::default(),
                      content: BeamValue::Continue
                    },
                  ],
                  metronome_tied: None,
                  metronome_tuplet: None
                }
              },
            ],
            additional: Some(AdditionalMetronomeBasedContents {
              metronome_relation: MetronomeRelation {
                attributes: (),
                content: String::from("Relation String")
              },
              metronome_note: vec![MetronomeNote {
                attributes: (),
                content: MetronomeNoteContents {
                  metronome_type: MetronomeType {
                    attributes: (),
                    content: NoteTypeValue::OneThousandTwentyFourth
                  },
                  metronome_dot: vec![],
                  metronome_beam: vec![],
                  metronome_tied: None,
                  metronome_tuplet: Some(MetronomeTuplet {
                    attributes: MetronomeTupletAttributes {
                      r#type: StartStop::Stop,
                      bracket: None,
                      show_number: None
                    },
                    content: MetronomeTupletContents {
                      actual_notes: ActualNotes {
                        attributes: (),
                        content: NonNegativeInteger(3)
                      },
                      normal_notes: NormalNotes {
                        attributes: (),
                        content: NonNegativeInteger(2)
                      },
                      normal_type: None,
                      normal_dot: vec![]
                    }
                  })
                }
              }]
            }),
          })
        },)
      }
    );
  }
}
