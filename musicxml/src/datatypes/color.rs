use musicxml_internal::{DatatypeDeserializer, DatatypeSerializer};
use regex::Regex;
use std::fmt;

/// Indicates the color of an element.
/// 
/// [Color] may be represented as hexadecimal RGB triples, as in HTML,
/// or as hexadecimal ARGB tuples, with the A indicating alpha of transparency.
/// An alpha value of `00` is totally transparent; `FF` is totally opaque.
/// If RGB is used, the A value is assumed to be `FF`.
/// 
/// For instance, the RGB value `#800080` represents purple.
/// An ARGB value of `#40800080` would be a transparent purple.
/// 
/// As in [SVG 1.1](https://www.w3.org/TR/SVG11/color.html),
/// colors are defined in terms of the [sRGB](https://www.color.org/srgb04.xalter) color space (IEC 61966).
#[derive(Debug, PartialEq, Eq)]
pub struct Color {
  /// Amount of red present in the color (0-127)
  pub r: u8,
  /// Amount of green present in the color (0-127)
  pub g: u8,
  /// Amount of blue present in the color (0-127)
  pub b: u8,
  /// Amount of transparency present in the color (0-127)
  pub a: u8,
}

impl Color {
  /// Creates a new [Color] with the given red, green, blue, and alpha values.
  pub fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
    Color { r, g, b, a }
  }
}

impl fmt::Display for Color {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "aRGB({}, {}, {}, {})", self.a, self.r, self.g, self.b)
  }
}

impl DatatypeSerializer for Color {
  fn serialize(element: &Self) -> String {
    if element.a > 0 {
      format!("#{:02X}{:02X}{:02X}{:02X}", element.a, element.r, element.g, element.b)
    } else {
      format!("#{:02X}{:02X}{:02X}", element.r, element.g, element.b)
    }
  }
}

impl DatatypeDeserializer for Color {
  fn deserialize(value: &str) -> Result<Self, String> {
    let regex = Regex::new(r"#([A-Fa-f0-9]{2})([A-Fa-f0-9]{2})([A-Fa-f0-9]{2})([A-Fa-f0-9]{2})?$").unwrap();
    if let Some(captures) = regex.captures(value) {
      if let Some(b) = captures.get(4) {
        Ok(Color::new(
          u8::from_str_radix(captures.get(2).unwrap().as_str(), 16).unwrap(),
          u8::from_str_radix(captures.get(3).unwrap().as_str(), 16).unwrap(),
          u8::from_str_radix(b.as_str(), 16).unwrap(),
          u8::from_str_radix(captures.get(1).unwrap().as_str(), 16).unwrap(),
        ))
      } else {
        Ok(Color::new(
          u8::from_str_radix(captures.get(1).unwrap().as_str(), 16).unwrap(),
          u8::from_str_radix(captures.get(2).unwrap().as_str(), 16).unwrap(),
          u8::from_str_radix(captures.get(3).unwrap().as_str(), 16).unwrap(),
          0,
        ))
      }
    } else {
      Err(format!("Value {} is invalid for the <color> data type", value))
    }
  }
}

#[cfg(test)]
mod color_tests {
  use super::*;

  #[test]
  fn serialize_valid1() {
    let test = Color::new(161, 35, 229, 0);
    let result = Color::serialize(&test);
    assert_eq!(result, "#A123E5");
  }

  #[test]
  fn serialize_valid2() {
    let test = Color::new(35, 229, 103, 161);
    let result = Color::serialize(&test);
    assert_eq!(result, "#A123E567");
  }

  #[test]
  fn deserialize_valid1() {
    let result = Color::deserialize("#A123e5");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), Color::new(161, 35, 229, 0));
  }

  #[test]
  fn deserialize_valid2() {
    let result = Color::deserialize("#A123e567");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), Color::new(35, 229, 103, 161));
  }

  #[test]
  fn deserialize_valid3() {
    let result = Color::deserialize("#abcdef");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), Color::new(171, 205, 239, 0));
  }

  #[test]
  fn deserialize_valid4() {
    let result = Color::deserialize("#fedcba23");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), Color::new(220, 186, 35, 254));
  }

  #[test]
  fn deserialize_invalid1() {
    let result = Color::deserialize("#A123e567a");
    assert!(result.is_err());
  }

  #[test]
  fn deserialize_invalid2() {
    let result = Color::deserialize("#A123e56");
    assert!(result.is_err());
  }

  #[test]
  fn deserialize_invalid3() {
    let result = Color::deserialize("#A123e");
    assert!(result.is_err());
  }

  #[test]
  fn deserialize_invalid4() {
    let result = Color::deserialize("#A123s2");
    assert!(result.is_err());
  }

  #[test]
  fn deserialize_invalid5() {
    let result = Color::deserialize("#A123efft");
    assert!(result.is_err());
  }

  #[test]
  fn deserialize_invalid6() {
    let result = Color::deserialize("a12346");
    assert!(result.is_err());
  }

  #[test]
  fn deserialize_invalid7() {
    let result = Color::deserialize("#G123e567");
    assert!(result.is_err());
  }
}
