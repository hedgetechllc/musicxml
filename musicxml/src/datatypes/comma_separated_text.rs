use alloc::{string::String, string::ToString, vec::Vec};
use core::ops::Deref;
use musicxml_internal::{DatatypeDeserializer, DatatypeSerializer};

/// Used to specify a comma-separated list of text elements, as is used by the `font_family` attribute.
///
/// The value of an instance of this type may be accessed by dereferencing the struct: `*datatype_val`.
#[derive(Debug, PartialEq, Eq)]
pub struct CommaSeparatedText(pub Vec<String>);

impl Deref for CommaSeparatedText {
  type Target = Vec<String>;
  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl DatatypeSerializer for CommaSeparatedText {
  fn serialize(element: &Self) -> String {
    element.0.join(",")
  }
}

impl DatatypeDeserializer for CommaSeparatedText {
  fn deserialize(value: &str) -> Result<Self, String> {
    let mut text: Vec<String> = Vec::new();
    value.split(',').for_each(|item| {
      let res = item.trim();
      if !res.is_empty() {
        text.push(res.to_string());
      }
    });
    Ok(CommaSeparatedText(text))
  }
}

#[cfg(test)]
mod comma_separated_text_tests {
  use super::*;

  #[test]
  fn deserialize_valid1() {
    let result = CommaSeparatedText::deserialize("Ariel");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), CommaSeparatedText(vec![String::from("Ariel")]));
  }

  #[test]
  fn deserialize_valid2() {
    let result = CommaSeparatedText::deserialize("Ariel,Times New Roman");
    assert!(result.is_ok());
    assert_eq!(
      result.unwrap(),
      CommaSeparatedText(vec![String::from("Ariel"), String::from("Times New Roman")])
    );
  }

  #[test]
  fn deserialize_valid3() {
    let result = CommaSeparatedText::deserialize("Ariel, Courier,Helvetica");
    assert!(result.is_ok());
    assert_eq!(
      result.unwrap(),
      CommaSeparatedText(vec![
        String::from("Ariel"),
        String::from("Courier"),
        String::from("Helvetica")
      ])
    );
  }

  #[test]
  fn deserialize_valid4() {
    let result = CommaSeparatedText::deserialize(",Ariel, Courier,Helvetica");
    assert!(result.is_ok());
    assert_eq!(
      result.unwrap(),
      CommaSeparatedText(vec![
        String::from("Ariel"),
        String::from("Courier"),
        String::from("Helvetica")
      ])
    );
  }

  #[test]
  fn deserialize_valid5() {
    let result = CommaSeparatedText::deserialize(",Ariel");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), CommaSeparatedText(vec![String::from("Ariel")]));
  }
}
