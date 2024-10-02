use alloc::string::{String, ToString};
use core::ops::Deref;
use musicxml_internal::{DatatypeDeserializer, DatatypeSerializer};
use musicxml_macros::DatatypeSerialize;

/// Distinguishes up to 16 concurrent objects of the same type when the objects overlap in MusicXML document order.
///
/// Values greater than 6 are usually only needed for music with a large number of divisi staves in a single part,
/// or if there are more than 6 cross-staff arpeggios in a single measure. When a number-level value is optional and
/// has no default value, it is 1 if not specified.
///
/// When polyphonic parts are involved, the ordering within a MusicXML document can differ from musical score order.
/// As an example, say we have a piano part in 4/4 where within a single measure, all the notes on the top staff are
/// followed by all the notes on the bottom staff. In this example, each staff has a slur that starts on beat 2 and
/// stops on beat 3, and there is a third slur that goes from beat 1 of one staff to beat 4 of the other staff.
///
/// In this situation, the two mid-measure slurs can use the same number because they do not overlap in MusicXML document
/// order, even though they do overlap in musical score order. Within the MusicXML document, the top staff slur will
/// both start and stop before the bottom staff slur starts and stops.
///
/// If the cross-staff slur starts in the top staff and stops in the bottom staff, it will need a separate number from
/// the mid-measure slurs because it overlaps those slurs in MusicXML document order. However, if the cross-staff slur
/// starts in the bottom staff and stops in the top staff, all three slurs can use the same number. None of them overlap
/// within the MusicXML document, even though they all overlap each other in the musical score order. Within the MusicXML
/// document, the start and stop of the top-staff slur will be followed by the stop and start of the cross-staff slur,
/// followed by the start and stop of the bottom-staff slur.
///
/// As this example demonstrates, a reading program should be prepared to handle cases where the number-levels start
/// and stop in an arbitrary order. Because the start and stop values refer to musical score order, a program may
/// find the stopping point of an object earlier in the MusicXML document than it will find its starting point.
///
/// The value of an instance of this type may be accessed by dereferencing the struct: `*datatype_val`.
///
/// **Minimum value**: 1
///
/// **Maximum value**: 16
#[derive(Debug, PartialEq, Eq, DatatypeSerialize)]
pub struct NumberLevel(pub u8);

impl Deref for NumberLevel {
  type Target = u8;
  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl DatatypeDeserializer for NumberLevel {
  fn deserialize(value: &str) -> Result<Self, String> {
    match value.parse::<u8>() {
      Ok(val) => match val {
        1..=16 => Ok(NumberLevel(val)),
        _ => Err(format!("Value {val} is invalid for the <number-level> data type")),
      },
      Err(err) => {
        if err.to_string().to_lowercase().contains("eof") {
          Ok(NumberLevel(1))
        } else {
          Err(format!("Value {value} is invalid for the <number-level> data type"))
        }
      }
    }
  }
}

#[cfg(test)]
mod number_level_tests {
  use super::*;

  #[test]
  fn deserialize_valid1() {
    let result = NumberLevel::deserialize("1");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), NumberLevel(1));
  }

  #[test]
  fn deserialize_valid2() {
    let result = NumberLevel::deserialize("16");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), NumberLevel(16));
  }

  #[test]
  fn deserialize_invalid1() {
    let result = NumberLevel::deserialize("0");
    assert!(result.is_err());
  }

  #[test]
  fn deserialize_invalid2() {
    let result = NumberLevel::deserialize("17");
    assert!(result.is_err());
  }

  #[test]
  fn deserialize_invalid3() {
    let result = NumberLevel::deserialize("-2");
    assert!(result.is_err());
  }

  #[test]
  fn deserialize_invalid4() {
    let result = NumberLevel::deserialize("");
    assert!(result.is_err());
  }
}
