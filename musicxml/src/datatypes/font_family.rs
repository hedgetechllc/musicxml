use alloc::{string::String, string::ToString, vec::Vec};
use core::ops::Deref;
use musicxml_internal::{DatatypeDeserializer, DatatypeSerializer};

/// Comma-separated list of font names.
///
/// These can be specific font styles such as Maestro or Opus, or one of several generic
/// font styles: music, engraved, handwritten, text, serif, sans-serif, handwritten, cursive,
/// fantasy, and monospace.
///
/// The music, engraved, and handwritten values refer to music fonts; the rest refer to text fonts.
/// The fantasy style refers to decorative text such as found in older German-style printing.
///
/// The value of an instance of this type may be accessed by dereferencing the struct: `*datatype_val`.
#[derive(Debug, PartialEq, Eq)]
pub struct FontFamily(pub Vec<String>);

impl Deref for FontFamily {
  type Target = Vec<String>;
  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl DatatypeSerializer for FontFamily {
  fn serialize(element: &Self) -> String {
    element.0.join(",")
  }
}

impl DatatypeDeserializer for FontFamily {
  fn deserialize(value: &str) -> Result<Self, String> {
    let mut res: Vec<String> = Vec::new();
    value.split(',').for_each(|item| {
      let val = item.trim();
      if val.len() > 0 {
        res.push(val.to_string());
      }
    });
    Ok(FontFamily(res))
  }
}

#[cfg(test)]
mod font_family_tests {
  use super::*;

  #[test]
  fn serialize_valid1() {
    let test = FontFamily(vec![
      String::from("Ariel"),
      String::from("Times New Roman"),
      String::from("Courier"),
    ]);
    let result = FontFamily::serialize(&test);
    assert_eq!(result, "Ariel,Times New Roman,Courier");
  }

  #[test]
  fn serialize_valid2() {
    let test = FontFamily(vec![String::from("Ariel")]);
    let result = FontFamily::serialize(&test);
    assert_eq!(result, "Ariel");
  }

  #[test]
  fn deserialize_valid1() {
    let result = FontFamily::deserialize("Ariel");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), FontFamily(vec![String::from("Ariel")]));
  }

  #[test]
  fn deserialize_valid2() {
    let result = FontFamily::deserialize("Ariel,Times New Roman");
    assert!(result.is_ok());
    assert_eq!(
      result.unwrap(),
      FontFamily(vec![String::from("Ariel"), String::from("Times New Roman")])
    );
  }

  #[test]
  fn deserialize_valid3() {
    let result = FontFamily::deserialize("Ariel, Courier,Helvetica");
    assert!(result.is_ok());
    assert_eq!(
      result.unwrap(),
      FontFamily(vec![
        String::from("Ariel"),
        String::from("Courier"),
        String::from("Helvetica")
      ])
    );
  }

  #[test]
  fn deserialize_valid4() {
    let result = FontFamily::deserialize(",Ariel, Courier,Helvetica");
    assert!(result.is_ok());
    assert_eq!(
      result.unwrap(),
      FontFamily(vec![
        String::from("Ariel"),
        String::from("Courier"),
        String::from("Helvetica")
      ])
    );
  }

  #[test]
  fn deserialize_valid5() {
    let result = FontFamily::deserialize(",Ariel");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), FontFamily(vec![String::from("Ariel")]));
  }
}
