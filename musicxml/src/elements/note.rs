use super::{
  Accidental, Beam, Chord, Cue, Dot, Duration, Footnote, Grace, Instrument, Level, Listen, Lyric, Notations, Notehead,
  NoteheadText, Pitch, Play, Rest, Staff, Stem, Tie, TimeModification, Type, Unpitched, Voice,
};
use crate::datatypes::{
  Color, Divisions, FontFamily, FontSize, FontStyle, FontWeight, Id, NonNegativeDecimal, Tenths, TimeOnly, YesNo,
};
use alloc::{string::String, vec::Vec};
use musicxml_internal::*;
use musicxml_macros::*;

/// Attributes pertaining to the [Note] element.
#[derive(Debug, Default, PartialEq, Eq, AttributeDeserialize, AttributeSerialize)]
pub struct NoteAttributes {
  /// Alters the starting time of the note from when it would otherwise occur based on the flow of durations -
  /// information that is specific to a performance. It is expressed in terms of divisions, either positive or negative.
  /// A [Note] that stops a tie should not have an `attack` attribute. The `attack` and `release` attributes are independent
  /// of each other. The `attack` attribute only changes the starting time of a note.
  pub attack: Option<Divisions>,
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
  /// Corresponds to MIDI 1.0's Note On velocity, expressed in terms of percentage of the default forte value (90 for MIDI 1.0).
  pub dynamics: Option<NonNegativeDecimal>,
  /// Corresponds to MIDI 1.0's Note Off velocity, expressed in terms of percentage of the default forte value (90 for MIDI 1.0).
  pub end_dynamics: Option<NonNegativeDecimal>,
  /// A comma-separated list of font names.
  pub font_family: Option<FontFamily>,
  /// One of the CSS sizes or a numeric point size.
  pub font_size: Option<FontSize>,
  /// Normal or italic style.
  pub font_style: Option<FontStyle>,
  /// Normal or bold weight.
  pub font_weight: Option<FontWeight>,
  /// Specifies an ID that is unique to the entire document.
  pub id: Option<Id>,
  /// Used when just this note is sounded pizzicato, vs. the Pizzicato element which changes overall playback between pizzicato and arco.
  pub pizzicato: Option<YesNo>,
  /// Controls the printing of an augmentation dot separately from the rest of the note or rest. This is especially useful for notes that overlap in
  /// different voices, or for chord sheets that contain lyrics and chords but no melody. If `print_object` is set to no, this attribute is also
  /// interpreted as being set to no if not present.
  pub print_dot: Option<YesNo>,
  /// Indicates whether leger lines are printed. Notes without leger lines are used to indicate indeterminate high and low notes. It is yes if not
  /// present unless `print_object` is set to no. This attribute is ignored for rests.
  pub print_leger: Option<YesNo>,
  /// Controls the printing of a lyric separately from the rest of the note or rest. This is especially useful for notes that overlap in different voices,
  /// or for chord sheets that contain lyrics and chords but no melody. If `print_object` is set to no, this attribute is also interpreted as being set to no if not present.
  pub print_lyric: Option<YesNo>,
  /// Specifies whether or not to print an object. It is yes if not specified.
  pub print_object: Option<YesNo>,
  /// Controls whether or not spacing is left for an invisible note or object. It is used only if no note, dot, or lyric is being printed.
  /// The value is yes (leave spacing) if not specified.
  pub print_spacing: Option<YesNo>,
  /// Changes the horizontal position relative to the default position, either as computed by the individual program, or as overridden by the `default_x` attribute.
  /// Positive x is right and negative x is left. It should be interpreted in the context of the [Offset][super::Offset] element or directive attribute if those are present.
  pub relative_x: Option<Tenths>,
  /// Changes the vertical position relative to the default position, either as computed by the individual program, or as overridden by the `default_y` attribute.
  /// Positive y is up and negative y is down. It should be interpreted in the context of the `placement` attribute if that is present.
  pub relative_y: Option<Tenths>,
  /// Alters the stopping time of the note from when it would otherwise occur based on the flow of durations - information that is specific to a performance.
  /// It is expressed in terms of divisions, either positive or negative. A [Note] that starts a tie should not have a `release` attribute. The `attack` and
  /// 1 attributes are independent of each other. The `release` attribute only changes the stopping time of a note.
  pub release: Option<Divisions>,
  /// Restricts the type to apply to a set of times through a repeated section.
  /// If missing, the type applies all times through the repeated section.
  pub time_only: Option<TimeOnly>,
}

/// The [AudibleType] element contains whether the note is a pitch, unpitched, or rest.
#[derive(Debug, PartialEq, Eq)]
pub enum AudibleType {
  /// The [Pitch] element contains the pitch of a note.
  Pitch(Pitch),
  /// The [Unpitched] element contains the unpitched information of a note.
  Unpitched(Unpitched),
  /// The [Rest] element contains the rest information of a note.
  Rest(Rest),
}

/// The [GraceNormalInfo] element contains the actual data for a normal grace note.
#[derive(Debug, PartialEq, Eq)]
pub struct GraceNormalInfo {
  /// The [Chord] element indicates that this note is a chord note.
  pub chord: Option<Chord>,
  /// The [AudibleType] element contains whether the note is a pitch, unpitched, or rest.
  pub audible: AudibleType,
  /// The [Tie] element is used to specify the tie of a note.
  pub tie: Vec<Tie>,
}

impl ContentDeserializer for GraceNormalInfo {
  fn deserialize(elements: &Vec<XmlElement>) -> Result<Self, String> {
    let mut chord: Option<Chord> = None;
    let mut audible: Option<AudibleType> = None;
    let mut tie: Vec<Tie> = Vec::new();
    for element in elements {
      match element.name.as_str() {
        "chord" => chord = Some(Chord::deserialize(element)?),
        "pitch" => audible = Some(AudibleType::Pitch(Pitch::deserialize(element)?)),
        "unpitched" => audible = Some(AudibleType::Unpitched(Unpitched::deserialize(element)?)),
        "rest" => audible = Some(AudibleType::Rest(Rest::deserialize(element)?)),
        "tie" => tie.push(Tie::deserialize(element)?),
        _ => {}
      }
    }
    Ok(GraceNormalInfo {
      chord: if let Some(chord) = chord { Some(chord) } else { None },
      audible: if let Some(audible) = audible {
        audible
      } else {
        Err("Missing <pitch>, <unpitched>, or <rest> element")?
      },
      tie,
    })
  }
}

impl ContentSerializer for GraceNormalInfo {
  fn serialize(element: &Self) -> Vec<XmlElement> {
    let mut elements: Vec<XmlElement> = Vec::new();
    if let Some(content) = &element.chord {
      elements.push(Chord::serialize(content));
    }
    match &element.audible {
      AudibleType::Pitch(content) => elements.push(Pitch::serialize(content)),
      AudibleType::Unpitched(content) => elements.push(Unpitched::serialize(content)),
      AudibleType::Rest(content) => elements.push(Rest::serialize(content)),
    }
    for el in &element.tie {
      elements.push(Tie::serialize(el));
    }
    elements
  }
}

/// The [GraceCueInfo] element contains the actual data for a cue grace note.
#[derive(Debug, PartialEq, Eq)]
pub struct GraceCueInfo {
  /// The [Cue] element indicates that this note is a cue note.
  pub cue: Cue,
  /// The [Chord] element indicates that this note is a chord note.
  pub chord: Option<Chord>,
  /// The [AudibleType] element contains whether the note is a pitch, unpitched, or rest.
  pub audible: AudibleType,
}

impl ContentDeserializer for GraceCueInfo {
  fn deserialize(elements: &Vec<XmlElement>) -> Result<Self, String> {
    let mut cue: Option<Cue> = None;
    let mut chord: Option<Chord> = None;
    let mut audible: Option<AudibleType> = None;
    for element in elements {
      match element.name.as_str() {
        "cue" => cue = Some(Cue::deserialize(element)?),
        "chord" => chord = Some(Chord::deserialize(element)?),
        "pitch" => audible = Some(AudibleType::Pitch(Pitch::deserialize(element)?)),
        "unpitched" => audible = Some(AudibleType::Unpitched(Unpitched::deserialize(element)?)),
        "rest" => audible = Some(AudibleType::Rest(Rest::deserialize(element)?)),
        _ => {}
      }
    }
    Ok(GraceCueInfo {
      cue: if let Some(cue) = cue {
        cue
      } else {
        Err("Missing <cue> element")?
      },
      chord: if let Some(chord) = chord { Some(chord) } else { None },
      audible: if let Some(audible) = audible {
        audible
      } else {
        Err("Missing <pitch>, <unpitched>, or <rest> element")?
      },
    })
  }
}

impl ContentSerializer for GraceCueInfo {
  fn serialize(element: &Self) -> Vec<XmlElement> {
    let mut elements: Vec<XmlElement> = Vec::new();
    elements.push(Cue::serialize(&element.cue));
    if let Some(content) = &element.chord {
      elements.push(Chord::serialize(content));
    }
    match &element.audible {
      AudibleType::Pitch(content) => elements.push(Pitch::serialize(content)),
      AudibleType::Unpitched(content) => elements.push(Unpitched::serialize(content)),
      AudibleType::Rest(content) => elements.push(Rest::serialize(content)),
    }
    elements
  }
}

/// The [GraceType] element contains the actual data for either a cue grace note or a normal grace note.
#[derive(Debug, PartialEq, Eq)]
pub enum GraceType {
  /// The [GraceCueInfo] element contains the actual data for a cue grace note.
  Cue(GraceCueInfo),
  /// The [GraceNormalInfo] element contains the actual data for a normal grace note.
  Normal(GraceNormalInfo),
}

/// The [GraceInfo] element contains the actual data for a grace note.
#[derive(Debug, PartialEq, Eq)]
pub struct GraceInfo {
  /// The [Grace] element indicates that this note is a grace note.
  pub grace: Grace,
  /// The [GraceType] element contains the actual data for the grace note.
  pub info: GraceType,
}

impl ContentDeserializer for GraceInfo {
  fn deserialize(elements: &Vec<XmlElement>) -> Result<Self, String> {
    Ok(GraceInfo {
      grace: Grace::deserialize(elements.first().unwrap())?,
      info: if elements.iter().find(|&el| el.name == "cue").is_some() {
        GraceType::Cue(GraceCueInfo::deserialize(elements)?)
      } else {
        GraceType::Normal(GraceNormalInfo::deserialize(elements)?)
      },
    })
  }
}

impl ContentSerializer for GraceInfo {
  fn serialize(element: &Self) -> Vec<XmlElement> {
    let mut elements: Vec<XmlElement> = Vec::new();
    elements.push(Grace::serialize(&element.grace));
    match &element.info {
      GraceType::Cue(content) => elements.extend(GraceCueInfo::serialize(content)),
      GraceType::Normal(content) => elements.extend(GraceNormalInfo::serialize(content)),
    }
    elements
  }
}

/// The [CueInfo] element contains the actual data for a cue note.
#[derive(Debug, PartialEq, Eq)]
pub struct CueInfo {
  /// The [Cue] element indicates that this note is a cue note.
  pub cue: Cue,
  /// The [Chord] element indicates that this note is a chord note.
  pub chord: Option<Chord>,
  /// The [AudibleType] element contains whether the note is a pitch, unpitched, or rest.
  pub audible: AudibleType,
  /// The [Duration] element indicates the duration of the note.
  pub duration: Duration,
}

impl ContentDeserializer for CueInfo {
  fn deserialize(elements: &Vec<XmlElement>) -> Result<Self, String> {
    let mut cue: Option<Cue> = None;
    let mut chord: Option<Chord> = None;
    let mut audible: Option<AudibleType> = None;
    let mut duration: Option<Duration> = None;
    for element in elements {
      match element.name.as_str() {
        "cue" => cue = Some(Cue::deserialize(element)?),
        "chord" => chord = Some(Chord::deserialize(element)?),
        "pitch" => audible = Some(AudibleType::Pitch(Pitch::deserialize(element)?)),
        "unpitched" => audible = Some(AudibleType::Unpitched(Unpitched::deserialize(element)?)),
        "rest" => audible = Some(AudibleType::Rest(Rest::deserialize(element)?)),
        "duration" => duration = Some(Duration::deserialize(element)?),
        _ => {}
      }
    }
    Ok(CueInfo {
      cue: if let Some(cue) = cue {
        cue
      } else {
        Err("Missing <cue> element")?
      },
      chord: if let Some(chord) = chord { Some(chord) } else { None },
      audible: if let Some(audible) = audible {
        audible
      } else {
        Err("Missing <pitch>, <unpitched>, or <rest> element")?
      },
      duration: if let Some(duration) = duration {
        duration
      } else {
        Err("Missing <duration> element")?
      },
    })
  }
}

impl ContentSerializer for CueInfo {
  fn serialize(element: &Self) -> Vec<XmlElement> {
    let mut elements: Vec<XmlElement> = Vec::new();
    elements.push(Cue::serialize(&element.cue));
    if let Some(content) = &element.chord {
      elements.push(Chord::serialize(content));
    }
    match &element.audible {
      AudibleType::Pitch(content) => elements.push(Pitch::serialize(content)),
      AudibleType::Unpitched(content) => elements.push(Unpitched::serialize(content)),
      AudibleType::Rest(content) => elements.push(Rest::serialize(content)),
    }
    elements.push(Duration::serialize(&element.duration));
    elements
  }
}

/// The [NormalInfo] element contains the actual data for a normal note.
#[derive(Debug, PartialEq, Eq)]
pub struct NormalInfo {
  /// The [Chord] element indicates that this note is a chord note.
  pub chord: Option<Chord>,
  /// The [AudibleType] element contains whether the note is a pitch, unpitched, or rest.
  pub audible: AudibleType,
  /// The [Duration] element indicates the duration of the note.
  pub duration: Duration,
  /// The [Tie] element is used to specify the tie of a note.
  pub tie: Vec<Tie>,
}

impl ContentDeserializer for NormalInfo {
  fn deserialize(elements: &Vec<XmlElement>) -> Result<Self, String> {
    let mut chord: Option<Chord> = None;
    let mut audible: Option<AudibleType> = None;
    let mut duration: Option<Duration> = None;
    let mut tie: Vec<Tie> = Vec::new();
    for element in elements {
      match element.name.as_str() {
        "chord" => chord = Some(Chord::deserialize(element)?),
        "pitch" => audible = Some(AudibleType::Pitch(Pitch::deserialize(element)?)),
        "unpitched" => audible = Some(AudibleType::Unpitched(Unpitched::deserialize(element)?)),
        "rest" => audible = Some(AudibleType::Rest(Rest::deserialize(element)?)),
        "duration" => duration = Some(Duration::deserialize(element)?),
        "tie" => tie.push(Tie::deserialize(element)?),
        _ => {}
      }
    }
    Ok(NormalInfo {
      chord: if let Some(chord) = chord { Some(chord) } else { None },
      audible: if let Some(audible) = audible {
        audible
      } else {
        Err("Missing <pitch>, <unpitched>, or <rest> element")?
      },
      duration: if let Some(duration) = duration {
        duration
      } else {
        Err("Missing <duration> element")?
      },
      tie,
    })
  }
}

impl ContentSerializer for NormalInfo {
  fn serialize(element: &Self) -> Vec<XmlElement> {
    let mut elements: Vec<XmlElement> = Vec::new();
    if let Some(content) = &element.chord {
      elements.push(Chord::serialize(content));
    }
    match &element.audible {
      AudibleType::Pitch(content) => elements.push(Pitch::serialize(content)),
      AudibleType::Unpitched(content) => elements.push(Unpitched::serialize(content)),
      AudibleType::Rest(content) => elements.push(Rest::serialize(content)),
    }
    elements.push(Duration::serialize(&element.duration));
    for el in &element.tie {
      elements.push(Tie::serialize(el));
    }
    elements
  }
}

/// The [NoteType] element contains the actual data for a note.
#[derive(Debug, PartialEq, Eq)]
pub enum NoteType {
  /// The [GraceInfo] element contains the actual data for a grace note.
  Grace(GraceInfo),
  /// The [CueInfo] element contains the actual data for a cue note.
  Cue(CueInfo),
  /// The [NormalInfo] element contains the actual data for a normal note.
  Normal(NormalInfo),
}

/// Contents of the [Note] element.
#[derive(Debug, PartialEq, Eq)]
pub struct NoteContents {
  /// The [NoteType] element contains the actual data for the note.
  pub info: NoteType,
  /// The [Instrument] element is used to specify the instrument for a note.
  pub instrument: Vec<Instrument>,
  /// The [Footnote] element is used to specify editorial information or analysis.
  pub footnote: Option<Footnote>,
  /// The [Level] element is used to specify editorial information or analysis.
  pub level: Option<Level>,
  /// The [Voice] element is used to specify the voice of a note.
  pub voice: Option<Voice>,
  /// The [Type] element is used to specify the type of a note.
  pub r#type: Option<Type>,
  /// The [Dot] element is used to specify the presence of a dot on a note.
  pub dot: Vec<Dot>,
  /// The [Accidental] element is used to specify the accidental of a note.
  pub accidental: Option<Accidental>,
  /// The [TimeModification] element is used to specify the time modification of a note.
  pub time_modification: Option<TimeModification>,
  /// The [Stem] element is used to specify the stem of a note.
  pub stem: Option<Stem>,
  /// The [Notehead] element is used to specify the notehead of a note.
  pub notehead: Option<Notehead>,
  /// The [NoteheadText] element is used to specify the notehead text of a note.
  pub notehead_text: Option<NoteheadText>,
  /// The [Staff] element is used to specify the staff of a note.
  pub staff: Option<Staff>,
  /// The [Beam] element is used to specify the beam of a note.
  pub beam: Vec<Beam>,
  /// The [Notations] element is used to specify the notations of a note.
  pub notations: Vec<Notations>,
  /// The [Lyric] element is used to specify the lyric of a note.
  pub lyric: Vec<Lyric>,
  /// The [Play] element is used to specify the play of a note.
  pub play: Option<Play>,
  /// The [Listen] element is used to specify the listen of a note.
  pub listen: Option<Listen>,
}

impl ContentDeserializer for NoteContents {
  fn deserialize(elements: &Vec<XmlElement>) -> Result<Self, String> {
    let mut note_contents = NoteContents {
      info: if elements.first().unwrap().name == "grace" {
        NoteType::Grace(GraceInfo::deserialize(elements)?)
      } else if elements.first().unwrap().name == "cue" {
        NoteType::Cue(CueInfo::deserialize(elements)?)
      } else {
        NoteType::Normal(NormalInfo::deserialize(elements)?)
      },
      instrument: vec![],
      footnote: None,
      level: None,
      voice: None,
      r#type: None,
      dot: vec![],
      accidental: None,
      time_modification: None,
      stem: None,
      notehead: None,
      notehead_text: None,
      staff: None,
      beam: vec![],
      notations: vec![],
      lyric: vec![],
      play: None,
      listen: None,
    };
    for element in elements {
      match element.name.as_str() {
        "instrument" => note_contents.instrument.push(Instrument::deserialize(element)?),
        "footnote" => note_contents.footnote = Some(Footnote::deserialize(element)?),
        "level" => note_contents.level = Some(Level::deserialize(element)?),
        "voice" => note_contents.voice = Some(Voice::deserialize(element)?),
        "type" => note_contents.r#type = Some(Type::deserialize(element)?),
        "dot" => note_contents.dot.push(Dot::deserialize(element)?),
        "accidental" => note_contents.accidental = Some(Accidental::deserialize(element)?),
        "time-modification" => note_contents.time_modification = Some(TimeModification::deserialize(element)?),
        "stem" => note_contents.stem = Some(Stem::deserialize(element)?),
        "notehead" => note_contents.notehead = Some(Notehead::deserialize(element)?),
        "notehead-text" => note_contents.notehead_text = Some(NoteheadText::deserialize(element)?),
        "staff" => note_contents.staff = Some(Staff::deserialize(element)?),
        "beam" => note_contents.beam.push(Beam::deserialize(element)?),
        "notations" => note_contents.notations.push(Notations::deserialize(element)?),
        "lyric" => note_contents.lyric.push(Lyric::deserialize(element)?),
        "play" => note_contents.play = Some(Play::deserialize(element)?),
        "listen" => note_contents.listen = Some(Listen::deserialize(element)?),
        _ => {}
      }
    }
    Ok(note_contents)
  }
}

impl ContentSerializer for NoteContents {
  fn serialize(element: &Self) -> Vec<XmlElement> {
    let mut elements: Vec<XmlElement> = Vec::new();
    match &element.info {
      NoteType::Grace(content) => elements.extend(GraceInfo::serialize(content)),
      NoteType::Cue(content) => elements.extend(CueInfo::serialize(content)),
      NoteType::Normal(content) => elements.extend(NormalInfo::serialize(content)),
    }
    for el in &element.instrument {
      elements.push(Instrument::serialize(el));
    }
    if let Some(content) = &element.footnote {
      elements.push(Footnote::serialize(content));
    }
    if let Some(content) = &element.level {
      elements.push(Level::serialize(content));
    }
    if let Some(content) = &element.voice {
      elements.push(Voice::serialize(content));
    }
    if let Some(content) = &element.r#type {
      elements.push(Type::serialize(content));
    }
    for content in &element.dot {
      elements.push(Dot::serialize(content));
    }
    if let Some(content) = &element.accidental {
      elements.push(Accidental::serialize(content));
    }
    if let Some(content) = &element.time_modification {
      elements.push(TimeModification::serialize(content));
    }
    if let Some(content) = &element.stem {
      elements.push(Stem::serialize(content));
    }
    if let Some(content) = &element.notehead {
      elements.push(Notehead::serialize(content));
    }
    if let Some(content) = &element.notehead_text {
      elements.push(NoteheadText::serialize(content));
    }
    if let Some(content) = &element.staff {
      elements.push(Staff::serialize(content));
    }
    for content in &element.beam {
      elements.push(Beam::serialize(content));
    }
    for content in &element.notations {
      elements.push(Notations::serialize(content));
    }
    for content in &element.lyric {
      elements.push(Lyric::serialize(content));
    }
    if let Some(content) = &element.play {
      elements.push(Play::serialize(content));
    }
    if let Some(content) = &element.listen {
      elements.push(Listen::serialize(content));
    }
    elements
  }
}

/// Notes are the most common type of MusicXML data.
///
/// The MusicXML format distinguishes between elements used for sound information and elements used for notation information
/// (e.g., [Tie] is used for sound, [Tied][super::Tied] for notation). Thus grace notes do not have a [Duration] element. Cue notes
/// have a [Duration] element, as do [Forward][super::Forward] elements, but no [Tie] elements. Having these two types of information
/// available can make interchange easier, as some programs handle one type of information more readily than the other.
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
pub struct Note {
  /// Element-specific attributes
  pub attributes: NoteAttributes,
  #[flatten]
  /// Element-specific content
  pub content: NoteContents,
}

#[cfg(test)]
mod note_tests {
  use super::*;
  use crate::datatypes::StartStop;
  use crate::elements::*;
  use crate::parser::parse_from_xml_str;
  use alloc::string::ToString;

  #[test]
  fn deserialize_valid1() {
    let result = parse_from_xml_str::<Note>(
      "<note attack=\"2\">
        <grace/>
        <chord/>
        <pitch>
          <step>C</step>
          <octave>4</octave>
        </pitch>
        <tie type=\"start\"/>
        <tie type=\"stop\"/>
      </note>",
    );
    assert!(result.is_ok());
    assert_eq!(
      result.unwrap(),
      Note {
        attributes: NoteAttributes {
          attack: Some(Divisions(2)),
          ..Default::default()
        },
        content: NoteContents {
          info: NoteType::Grace(GraceInfo {
            grace: Grace {
              attributes: GraceAttributes::default(),
              content: ()
            },
            info: GraceType::Normal(GraceNormalInfo {
              chord: Some(Chord {
                attributes: (),
                content: ()
              }),
              audible: AudibleType::Pitch(Pitch {
                attributes: (),
                content: PitchContents {
                  step: Step {
                    attributes: (),
                    content: crate::datatypes::Step::C
                  },
                  alter: None,
                  octave: Octave {
                    attributes: (),
                    content: crate::datatypes::Octave(4)
                  },
                }
              }),
              tie: vec![
                Tie {
                  attributes: TieAttributes {
                    r#type: StartStop::Start,
                    time_only: None
                  },
                  content: ()
                },
                Tie {
                  attributes: TieAttributes {
                    r#type: StartStop::Stop,
                    time_only: None
                  },
                  content: ()
                },
              ],
            }),
          }),
          instrument: vec![],
          footnote: None,
          level: None,
          voice: None,
          r#type: None,
          dot: vec![],
          accidental: None,
          time_modification: None,
          stem: None,
          notehead: None,
          notehead_text: None,
          staff: None,
          beam: vec![],
          notations: vec![],
          lyric: vec![],
          play: None,
          listen: None,
        },
      }
    );
  }

  #[test]
  fn deserialize_valid2() {
    let result = parse_from_xml_str::<Note>(
      "<note>
        <grace/>
        <cue/>
        <unpitched>
          <display-step>C</display-step>
          <display-octave>4</display-octave>
        </unpitched>
      </note>",
    );
    assert!(result.is_ok());
    assert_eq!(
      result.unwrap(),
      Note {
        attributes: NoteAttributes::default(),
        content: NoteContents {
          info: NoteType::Grace(GraceInfo {
            grace: Grace {
              attributes: GraceAttributes::default(),
              content: ()
            },
            info: GraceType::Cue(GraceCueInfo {
              cue: Cue {
                attributes: (),
                content: ()
              },
              chord: None,
              audible: AudibleType::Unpitched(Unpitched {
                attributes: (),
                content: UnpitchedContents {
                  display_step: DisplayStep {
                    attributes: (),
                    content: crate::datatypes::Step::C
                  },
                  display_octave: DisplayOctave {
                    attributes: (),
                    content: crate::datatypes::Octave(4)
                  },
                }
              }),
            }),
          }),
          instrument: vec![],
          footnote: None,
          level: None,
          voice: None,
          r#type: None,
          dot: vec![],
          accidental: None,
          time_modification: None,
          stem: None,
          notehead: None,
          notehead_text: None,
          staff: None,
          beam: vec![],
          notations: vec![],
          lyric: vec![],
          play: None,
          listen: None,
        },
      }
    );
  }

  #[test]
  fn deserialize_valid3() {
    let result = parse_from_xml_str::<Note>(
      "<note>
        <cue/>
        <unpitched>
          <display-step>C</display-step>
          <display-octave>4</display-octave>
        </unpitched>
        <duration>4</duration>
      </note>",
    );
    assert!(result.is_ok());
    assert_eq!(
      result.unwrap(),
      Note {
        attributes: NoteAttributes::default(),
        content: NoteContents {
          info: NoteType::Cue(CueInfo {
            cue: Cue {
              attributes: (),
              content: ()
            },
            chord: None,
            audible: AudibleType::Unpitched(Unpitched {
              attributes: (),
              content: UnpitchedContents {
                display_step: DisplayStep {
                  attributes: (),
                  content: crate::datatypes::Step::C
                },
                display_octave: DisplayOctave {
                  attributes: (),
                  content: crate::datatypes::Octave(4)
                },
              }
            }),
            duration: Duration {
              attributes: (),
              content: crate::datatypes::PositiveDivisions(4)
            }
          }),
          instrument: vec![],
          footnote: None,
          level: None,
          voice: None,
          r#type: None,
          dot: vec![],
          accidental: None,
          time_modification: None,
          stem: None,
          notehead: None,
          notehead_text: None,
          staff: None,
          beam: vec![],
          notations: vec![],
          lyric: vec![],
          play: None,
          listen: None,
        },
      }
    );
  }

  #[test]
  fn deserialize_valid4() {
    let result = parse_from_xml_str::<Note>(
      "<note>
        <rest>
          <display-step>C</display-step>
          <display-octave>4</display-octave>
        </rest>
        <duration>4</duration>
        <tie type=\"start\"/>
      </note>",
    );
    assert!(result.is_ok());
    assert_eq!(
      result.unwrap(),
      Note {
        attributes: NoteAttributes::default(),
        content: NoteContents {
          info: NoteType::Normal(NormalInfo {
            chord: None,
            audible: AudibleType::Rest(Rest {
              attributes: RestAttributes::default(),
              content: RestContents {
                display_step: Some(DisplayStep {
                  attributes: (),
                  content: crate::datatypes::Step::C
                }),
                display_octave: Some(DisplayOctave {
                  attributes: (),
                  content: crate::datatypes::Octave(4)
                }),
              }
            }),
            duration: Duration {
              attributes: (),
              content: crate::datatypes::PositiveDivisions(4)
            },
            tie: vec![Tie {
              attributes: TieAttributes {
                r#type: StartStop::Start,
                time_only: None
              },
              content: ()
            }],
          }),
          instrument: vec![],
          footnote: None,
          level: None,
          voice: None,
          r#type: None,
          dot: vec![],
          accidental: None,
          time_modification: None,
          stem: None,
          notehead: None,
          notehead_text: None,
          staff: None,
          beam: vec![],
          notations: vec![],
          lyric: vec![],
          play: None,
          listen: None,
        },
      }
    );
  }

  #[test]
  fn deserialize_valid5() {
    let result = parse_from_xml_str::<Note>(
      "<note>
        <grace/>
        <chord/>
        <pitch>
          <step>C</step>
          <octave>4</octave>
        </pitch>
        <tie type=\"start\"/>
        <tie type=\"stop\"/>
        <instrument id=\"I1\"/>
        <instrument id=\"I2\"/>
        <notehead>fa up</notehead>
      </note>",
    );
    assert!(result.is_ok());
    assert_eq!(
      result.unwrap(),
      Note {
        attributes: NoteAttributes::default(),
        content: NoteContents {
          info: NoteType::Grace(GraceInfo {
            grace: Grace {
              attributes: GraceAttributes::default(),
              content: ()
            },
            info: GraceType::Normal(GraceNormalInfo {
              chord: Some(Chord {
                attributes: (),
                content: ()
              }),
              audible: AudibleType::Pitch(Pitch {
                attributes: (),
                content: PitchContents {
                  step: Step {
                    attributes: (),
                    content: crate::datatypes::Step::C
                  },
                  alter: None,
                  octave: Octave {
                    attributes: (),
                    content: crate::datatypes::Octave(4)
                  },
                }
              }),
              tie: vec![
                Tie {
                  attributes: TieAttributes {
                    r#type: StartStop::Start,
                    time_only: None
                  },
                  content: ()
                },
                Tie {
                  attributes: TieAttributes {
                    r#type: StartStop::Stop,
                    time_only: None
                  },
                  content: ()
                },
              ],
            }),
          }),
          instrument: vec![
            Instrument {
              attributes: InstrumentAttributes {
                id: crate::datatypes::IdRef("I1".to_string())
              },
              content: ()
            },
            Instrument {
              attributes: InstrumentAttributes {
                id: crate::datatypes::IdRef("I2".to_string())
              },
              content: ()
            },
          ],
          footnote: None,
          level: None,
          voice: None,
          r#type: None,
          dot: vec![],
          accidental: None,
          time_modification: None,
          stem: None,
          notehead: Some(Notehead {
            attributes: NoteheadAttributes::default(),
            content: crate::datatypes::NoteheadValue::FaUp
          }),
          notehead_text: None,
          staff: None,
          beam: vec![],
          notations: vec![],
          lyric: vec![],
          play: None,
          listen: None,
        },
      }
    );
  }
}
