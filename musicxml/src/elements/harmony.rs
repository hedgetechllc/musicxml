use super::{Bass, Degree, Footnote, Frame, Function, Inversion, Kind, KindAttributes, Level, Numeral, Offset, Root, Staff};
use crate::datatypes::{
  AboveBelow, Color, FontFamily, FontSize, FontStyle, FontWeight, HarmonyArrangement, HarmonyType, Id, SystemRelation,
  Tenths, YesNo, KindValue,
};
use musicxml_internal::*;
use musicxml_macros::*;

/// Attributes pertaining to the [Harmony] element.
#[derive(Debug, Default, PartialEq, Eq, AttributeDeserialize, AttributeSerialize)]
pub struct HarmonyAttributes {
  /// Specifies how multiple harmony-chords are arranged relative to each other.
  /// Harmony-chords with vertical arrangement are separated by horizontal lines.
  /// Harmony-chords with diagonal or horizontal arrangement are separated by diagonal lines or slashes.
  pub arrangement: Option<HarmonyArrangement>,
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
  /// Indicates whether something is above or below another element, such as a note or a notation.
  pub placement: Option<AboveBelow>,
  /// Specifies the printing of a frame or fretboard diagram.
  pub print_frame: Option<YesNo>,
  /// Specifies whether or not to print an object. It is yes if not specified.
  pub print_object: Option<YesNo>,
  /// Changes the horizontal position relative to the default position, either as computed by the individual program, or as overridden by the `default_x` attribute.
  /// Positive x is right and negative x is left. It should be interpreted in the context of the [Offset][super::Offset] element or directive attribute if those are present.
  pub relative_x: Option<Tenths>,
  /// Changes the vertical position relative to the default position, either as computed by the individual program, or as overridden by the `default_y` attribute.
  /// Positive y is up and negative y is down. It should be interpreted in the context of the `placement` attribute if that is present.
  pub relative_y: Option<Tenths>,
  /// Distinguishes elements that are associated with a system rather than the particular part where the element appears.
  pub system: Option<SystemRelation>,
  /// If there are alternate harmonies possible, this can be specified using multiple [Harmony] elements differentiated by type.
  /// Explicit harmonies have all note present in the music; implied have some notes missing but implied; alternate represents alternate analyses.
  pub r#type: Option<HarmonyType>,
}

#[derive(Debug, PartialEq, Eq, ContentSerialize)]
pub struct HarmonySubcontents {
  pub root: Option<Root>,
  pub numeral: Option<Numeral>,
  pub function: Option<Function>,
  pub kind: Kind,
  pub inversion: Option<Inversion>,
  pub bass: Option<Bass>,
  pub degree: Vec<Degree>,
}

#[derive(Debug, Default, PartialEq, Eq)]
pub struct HarmonyContents {
  pub harmony: Vec<HarmonySubcontents>,
  pub frame: Option<Frame>,
  pub offset: Option<Offset>,
  pub footnote: Option<Footnote>,
  pub level: Option<Level>,
  pub staff: Option<Staff>,
}

impl ContentDeserializer for HarmonyContents {
  fn deserialize(elements: &Vec<XmlElement>) -> Result<Self, String> {
    let mut harmony: Vec<HarmonySubcontents> = Vec::new();
    for element in elements {
      if element.name == "root" || element.name == "numeral" || element.name == "function" {
        harmony.push(HarmonySubcontents {
          root: None,
          numeral: None,
          function: None,
          kind: Kind { attributes: KindAttributes::default(), content: KindValue::Major },
          inversion: None,
          bass: None,
          degree: Vec::new(),
        });
      }
      if let Some(sub) = harmony.last_mut() {
        match element.name.as_str() {
          "root" => sub.root = Some(Root::deserialize(element)?),
          "numeral" => sub.numeral = Some(Numeral::deserialize(element)?),
          "function" => sub.function = Some(Function::deserialize(element)?),
          "kind" => sub.kind = Kind::deserialize(element)?,
          "inversion" => sub.inversion = Some(Inversion::deserialize(element)?),
          "bass" => sub.bass = Some(Bass::deserialize(element)?),
          "degree" => sub.degree.push(Degree::deserialize(element)?),
          _ => (),
        }
      }
    }
    Ok(HarmonyContents {
      harmony,
      frame: match elements.iter().find(|&el| el.name == "frame") {
        Some(element) => Some(Frame::deserialize(element)?),
        None => None,
      },
      offset: match elements.iter().find(|&el| el.name == "offset") {
        Some(element) => Some(Offset::deserialize(element)?),
        None => None,
      },
      footnote: match elements.iter().find(|&el| el.name == "footnote") {
        Some(element) => Some(Footnote::deserialize(element)?),
        None => None,
      },
      level: match elements.iter().find(|&el| el.name == "level") {
        Some(element) => Some(Level::deserialize(element)?),
        None => None,
      },
      staff: match elements.iter().find(|&el| el.name == "staff") {
        Some(element) => Some(Staff::deserialize(element)?),
        None => None,
      },
    })
  }
}

impl ContentSerializer for HarmonyContents {
  fn serialize(element: &Self) -> Vec<XmlElement> {
    let mut elements: Vec<XmlElement> = Vec::new();
    for el in &element.harmony {
      elements.extend(HarmonySubcontents::serialize(el));
    }
    if let Some(el) = &element.frame {
      elements.push(Frame::serialize(el));
    }
    if let Some(el) = &element.offset {
      elements.push(Offset::serialize(el));
    }
    if let Some(el) = &element.footnote {
      elements.push(Footnote::serialize(el));
    }
    if let Some(el) = &element.level {
      elements.push(Level::serialize(el));
    }
    if let Some(el) = &element.staff {
      elements.push(Staff::serialize(el));
    }
    elements
  }
}

/// The [Harmony] element represents harmony analysis, including chord symbols in popular music as well as functional harmony analysis in classical music.
/// 
/// The `print_object` attribute controls whether or not anything is printed due to the [Harmony] element. The print suggestion attributes set the defaults
/// for the harmony, but individual elements can override this with their own values.
/// 
/// A [Harmony] element can contain many stacked chords (e.g. V of II). Each individual chord including a required [Kind] element is referred to as a harmony-chord.
/// Stacked chords or secondary functions are represented using a sequence of harmony-chords. For example, V of II would be represented by a harmony-chord
/// with a 5 numeral followed by a harmony-chord with a 2 numeral.
/// 
/// A [Root] is a pitch name like C, D, E, while a [Numeral] is a scale degree like 1, 2, 3. The [Root] element is generally used with pop chord symbols,
/// while the [Numeral] element is generally used with classical functional harmony and Nashville numbers. It is an either/or choice to avoid data inconsistency.
/// The [Function] element, which represents Roman numerals with roman numeral text, has been deprecated as of MusicXML 4.0.
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
pub struct Harmony {
  /// Element-specific attributes
  pub attributes: HarmonyAttributes,
  #[flatten]
  /// Element-specific content
  pub content: HarmonyContents,
}

#[cfg(test)]
mod harmony_tests {
  use super::*;
  use crate::elements::*;
  use crate::datatypes::{DegreeTypeValue, Divisions, KindValue, NonNegativeInteger, PositiveInteger, Semitones, StartStop, Step};
  use crate::parser::{parse_from_xml_str, parse_to_xml_str};

  #[test]
  fn serialize_valid1() {
    let test = Harmony {
      attributes: HarmonyAttributes {
        arrangement: Some(HarmonyArrangement::Diagonal),
        ..Default::default()
      },
      content: HarmonyContents {
        harmony: vec![
          HarmonySubcontents {
            root: Some(Root { attributes: (), content: RootContents { root_step: RootStep { attributes: RootStepAttributes::default(), content: Step::D }, root_alter: Some(RootAlter { attributes: RootAlterAttributes::default(), content: Semitones(-2) }) } }),
            numeral: None,
            function: None,
            kind: Kind { attributes: KindAttributes::default(), content: KindValue::Dominant11th },
            inversion: Some(Inversion{ attributes: InversionAttributes::default(), content: NonNegativeInteger(1) }),
            bass: Some(Bass { attributes: BassAttributes::default(), content: BassContents { bass_separator: Some(BassSeparator{ attributes: BassSeparatorAttributes::default(), content: String::from("Test Bass") }), bass_step: BassStep { attributes: BassStepAttributes::default(), content: Step::F }, bass_alter: None } }),
            degree: vec![
              Degree {
                attributes: DegreeAttributes::default(),
                content: DegreeContents {
                  degree_value: DegreeValue { attributes: DegreeValueAttributes::default(), content: PositiveInteger(5) },
                  degree_alter: DegreeAlter { attributes: DegreeAlterAttributes::default(), content: Semitones(7) },
                  degree_type: DegreeType { attributes: DegreeTypeAttributes::default(), content: DegreeTypeValue::Alter },
                }
              }
            ],
          }
        ],
        frame: Some(Frame {
          attributes: FrameAttributes::default(),
          content: FrameContents {
            frame_strings: FrameStrings { attributes: (), content: PositiveInteger(2) },
            frame_frets: FrameFrets { attributes: (), content: PositiveInteger(3) },
            first_fret: Some(FirstFret { attributes: FirstFretAttributes::default(), content: PositiveInteger(1) }),
            frame_note: vec![
              FrameNote {
                attributes: (),
                content: FrameNoteContents {
                  string: StringNumber{ attributes: StringAttributes::default(), content: crate::datatypes::StringNumber(2) },
                  fret: Fret{ attributes: FretAttributes::default(), content: NonNegativeInteger(1) },
                  fingering: Some(Fingering{ attributes: FingeringAttributes:: default(), content: String::from("Fingers") }),
                  barre: Some(Barre { attributes: BarreAttributes { r#type: StartStop::Start, color: None }, content: () }),
                }
              }
            ],
          },
        }),
        offset: Some(Offset {
          attributes: OffsetAttributes::default(),
          content: Divisions(7),
        }),
        footnote: None,
        level: None,
        staff: Some(Staff {
          attributes: (),
          content: PositiveInteger(2)
        }),
      }
    };
    let expected = "<harmony arrangement=\"diagonal\">
  <root>
    <root-step>D</root-step>
    <root-alter>-2</root-alter>
  </root>
  <kind>dominant-11th</kind>
  <inversion>1</inversion>
  <bass>
    <bass-separator>Test Bass</bass-separator>
    <bass-step>F</bass-step>
  </bass>
  <degree>
    <degree-value>5</degree-value>
    <degree-alter>7</degree-alter>
    <degree-type>alter</degree-type>
  </degree>
  <frame>
    <frame-strings>2</frame-strings>
    <frame-frets>3</frame-frets>
    <first-fret>1</first-fret>
    <frame-note>
      <string>2</string>
      <fret>1</fret>
      <fingering>Fingers</fingering>
      <barre type=\"start\"/>
    </frame-note>
  </frame>
  <offset>7</offset>
  <staff>2</staff>
</harmony>";
    let result = parse_to_xml_str(&test, true);
    assert_eq!(result, expected);
  }

  #[test]
  fn deserialize_valid1() {
    let result = parse_from_xml_str::<Harmony>(
      "<harmony arrangement=\"diagonal\">
        <root>
          <root-step>D</root-step>
          <root-alter>-2</root-alter>
        </root>
        <kind>dominant-11th</kind>
        <inversion>1</inversion>
        <bass>
          <bass-separator>Test Bass</bass-separator>
          <bass-step>F</bass-step>
        </bass>
        <degree>
          <degree-value>5</degree-value>
          <degree-alter>7</degree-alter>
          <degree-type>alter</degree-type>
        </degree>
        <frame>
          <frame-strings>2</frame-strings>
          <frame-frets>3</frame-frets>
          <first-fret>1</first-fret>
          <frame-note>
            <string>2</string>
            <fret>1</fret>
            <fingering>Fingers</fingering>
            <barre type=\"start\"/>
          </frame-note>
        </frame>
        <offset>7</offset>
        <staff>2</staff>
      </harmony>"
    );
    assert!(result.is_ok());
    assert_eq!(
      result.unwrap(),
      Harmony {
        attributes: HarmonyAttributes {
          arrangement: Some(HarmonyArrangement::Diagonal),
          ..Default::default()
        },
        content: HarmonyContents {
          harmony: vec![
            HarmonySubcontents {
              root: Some(Root { attributes: (), content: RootContents { root_step: RootStep { attributes: RootStepAttributes::default(), content: Step::D }, root_alter: Some(RootAlter { attributes: RootAlterAttributes::default(), content: Semitones(-2) }) } }),
              numeral: None,
              function: None,
              kind: Kind { attributes: KindAttributes::default(), content: KindValue::Dominant11th },
              inversion: Some(Inversion{ attributes: InversionAttributes::default(), content: NonNegativeInteger(1) }),
              bass: Some(Bass { attributes: BassAttributes::default(), content: BassContents { bass_separator: Some(BassSeparator{ attributes: BassSeparatorAttributes::default(), content: String::from("Test Bass") }), bass_step: BassStep { attributes: BassStepAttributes::default(), content: Step::F }, bass_alter: None } }),
              degree: vec![
                Degree {
                  attributes: DegreeAttributes::default(),
                  content: DegreeContents {
                    degree_value: DegreeValue { attributes: DegreeValueAttributes::default(), content: PositiveInteger(5) },
                    degree_alter: DegreeAlter { attributes: DegreeAlterAttributes::default(), content: Semitones(7) },
                    degree_type: DegreeType { attributes: DegreeTypeAttributes::default(), content: DegreeTypeValue::Alter },
                  }
                }
              ],
            }
          ],
          frame: Some(Frame {
            attributes: FrameAttributes::default(),
            content: FrameContents {
              frame_strings: FrameStrings { attributes: (), content: PositiveInteger(2) },
              frame_frets: FrameFrets { attributes: (), content: PositiveInteger(3) },
              first_fret: Some(FirstFret { attributes: FirstFretAttributes::default(), content: PositiveInteger(1) }),
              frame_note: vec![
                FrameNote {
                  attributes: (),
                  content: FrameNoteContents {
                    string: StringNumber{ attributes: StringAttributes::default(), content: crate::datatypes::StringNumber(2) },
                    fret: Fret{ attributes: FretAttributes::default(), content: NonNegativeInteger(1) },
                    fingering: Some(Fingering{ attributes: FingeringAttributes:: default(), content: String::from("Fingers") }),
                    barre: Some(Barre { attributes: BarreAttributes { r#type: StartStop::Start, color: None }, content: () }),
                  }
                }
              ],
            },
          }),
          offset: Some(Offset {
            attributes: OffsetAttributes::default(),
            content: Divisions(7),
          }),
          footnote: None,
          level: None,
          staff: Some(Staff {
            attributes: (),
            content: PositiveInteger(2)
          }),
        }
      }
    );
  }
}
