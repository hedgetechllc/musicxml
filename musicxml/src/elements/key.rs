use super::{Cancel, Fifths, KeyAccidental, KeyAlter, KeyOctave, KeyStep, Mode};
use crate::datatypes::{Color, FontFamily, FontSize, FontStyle, FontWeight, Id, StaffNumber, Tenths, YesNo};
use alloc::{string::String, vec::Vec};
use musicxml_internal::*;
use musicxml_macros::*;

/// Attributes pertaining to the [Key] element.
#[derive(Debug, Default, PartialEq, Eq, AttributeDeserialize, AttributeSerialize)]
pub struct KeyAttributes {
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
  /// Allows a key signature to apply to only the specified staff in the part. If absent, the key signature applies to all staves in the part.
  pub number: Option<StaffNumber>,
  /// Specifies whether or not to print an object. It is yes if not specified.
  pub print_object: Option<YesNo>,
  /// Changes the horizontal position relative to the default position, either as computed by the individual program, or as overridden by the `default_x` attribute.
  /// Positive x is right and negative x is left. It should be interpreted in the context of the [Offset][super::Offset] element or directive attribute if those are present.
  pub relative_x: Option<Tenths>,
  /// Changes the vertical position relative to the default position, either as computed by the individual program, or as overridden by the `default_y` attribute.
  /// Positive y is up and negative y is down. It should be interpreted in the context of the `placement` attribute if that is present.
  pub relative_y: Option<Tenths>,
}

/// Contents of the [ExplicitKeyContents] element.
#[derive(Debug, PartialEq, Eq, ContentDeserialize, ContentSerialize)]
pub struct ExplicitKeyContents {
  /// The [Cancel] element indicates the cancellation of a previous key signature.
  pub cancel: Option<Cancel>,
  /// The [Fifths] element represents the number of flats or sharps in the key signature.
  pub fifths: Fifths,
  /// The [Mode] element represents the mode of the key signature.
  pub mode: Option<Mode>,
  /// The [KeyOctave] element represents the octave of the key signature.
  pub key_octave: Vec<KeyOctave>,
}

/// Contents of the [RelativeKeyContents] element.
#[derive(Debug, PartialEq, Eq, ContentDeserialize, ContentSerialize)]
pub struct RelativeKeyContents {
  /// The [KeyStep] element represents the pitch step of the key signature.
  pub key_step: KeyStep,
  /// The [KeyAlter] element represents the alteration of the key signature.
  pub key_alter: KeyAlter,
  /// The [KeyAccidental] element represents the accidental of the key signature.
  pub key_accidental: Option<KeyAccidental>,
  /// The [KeyOctave] element represents the octave of the key signature.
  pub key_octave: Vec<KeyOctave>,
}

/// Contents of the [Key] element.
///
/// The [Key] element may contain either [ExplicitKeyContents] or [RelativeKeyContents].
#[derive(Debug, PartialEq, Eq)]
pub enum KeyContents {
  /// The [ExplicitKeyContents] element represents a key signature with a specified number of flats or sharps.
  Explicit(ExplicitKeyContents),
  /// The [RelativeKeyContents] element represents a key signature with a specified pitch step and alteration.
  Relative(RelativeKeyContents),
}

impl ContentDeserializer for KeyContents {
  fn deserialize(elements: &[XmlElement]) -> Result<Self, String> {
    Ok(if let Some(_) = elements.iter().find(|&el| el.name == "fifths") {
      KeyContents::Explicit(ExplicitKeyContents::deserialize(elements)?)
    } else {
      KeyContents::Relative(RelativeKeyContents::deserialize(elements)?)
    })
  }
}

impl ContentSerializer for KeyContents {
  fn serialize(element: &Self) -> Vec<XmlElement> {
    match &element {
      KeyContents::Explicit(content) => ExplicitKeyContents::serialize(content),
      KeyContents::Relative(content) => RelativeKeyContents::serialize(content),
    }
  }
}

/// The [Key] element represents a key signature.
///
/// Both traditional and non-traditional key signatures are supported. Key signatures appear at the start of each system
/// unless the `print_object` attribute has been set to "no".
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
pub struct Key {
  /// Element-specific attributes
  pub attributes: KeyAttributes,
  #[flatten]
  /// Element-specific content
  pub content: KeyContents,
}

#[cfg(test)]
mod key_tests {
  use super::*;
  use crate::datatypes::{Color, Octave, PositiveInteger, YesNo};
  use crate::elements::{
    Fifths, Key, KeyAccidentalAttributes, KeyAttributes, KeyContents, KeyOctave, KeyOctaveAttributes, Mode,
  };
  use crate::parser::parse_from_xml_str;

  #[test]
  fn deserialize_valid1() {
    let result = parse_from_xml_str::<Key>(
      "<key color=\"#123456\" print-object=\"no\">
        <fifths>2</fifths>
        <mode>major</mode>
        <key-octave number=\"1\">4</key-octave>
        <key-octave number=\"2\">3</key-octave>
      </key>",
    );
    assert!(result.is_ok());
    assert_eq!(
      result.unwrap(),
      Key {
        attributes: KeyAttributes {
          color: Some(Color::new(18, 52, 86, 0)),
          print_object: Some(YesNo::No),
          ..Default::default()
        },
        content: KeyContents::Explicit(ExplicitKeyContents {
          cancel: None,
          fifths: Fifths {
            attributes: (),
            content: crate::datatypes::Fifths(2)
          },
          mode: Some(Mode {
            attributes: (),
            content: crate::datatypes::Mode::Major
          }),
          key_octave: vec![
            KeyOctave {
              attributes: KeyOctaveAttributes {
                number: PositiveInteger(1),
                cancel: None
              },
              content: Octave(4)
            },
            KeyOctave {
              attributes: KeyOctaveAttributes {
                number: PositiveInteger(2),
                cancel: None
              },
              content: Octave(3)
            },
          ],
        }),
      }
    );
  }

  #[test]
  fn deserialize_valid2() {
    let result = parse_from_xml_str::<Key>(
      "<key>
        <key-step>C</key-step>
        <key-alter>1</key-alter>
        <key-accidental>sharp</key-accidental>
      </key>",
    );
    assert!(result.is_ok());
    assert_eq!(
      result.unwrap(),
      Key {
        attributes: KeyAttributes::default(),
        content: KeyContents::Relative(RelativeKeyContents {
          key_step: KeyStep {
            attributes: (),
            content: crate::datatypes::Step::C
          },
          key_alter: KeyAlter {
            attributes: (),
            content: crate::datatypes::Semitones(1)
          },
          key_accidental: Some(KeyAccidental {
            attributes: KeyAccidentalAttributes::default(),
            content: crate::datatypes::AccidentalValue::Sharp
          }),
          key_octave: Vec::new(),
        }),
      }
    );
  }
}
