use super::css_font_size::CssFontSize;
use alloc::string::{String, ToString};
use musicxml_internal::{DatatypeDeserializer, DatatypeSerializer};

/// Can be one of the [CSS font sizes](https://www.w3.org/2021/06/musicxml40/musicxml-reference/data-types/css-font-size/)
/// or a [decimal](https://www.w3.org/2021/06/musicxml40/musicxml-reference/data-types/xsd-decimal/) point size.
#[derive(Debug, PartialEq)]
pub enum FontSize {
  /// One of the [CSS font sizes](https://www.w3.org/2021/06/musicxml40/musicxml-reference/data-types/css-font-size/).
  Css(CssFontSize),
  /// A [decimal](https://www.w3.org/2021/06/musicxml40/musicxml-reference/data-types/xsd-decimal/) point size.
  Decimal(f32),
}

impl Eq for FontSize {}

impl DatatypeSerializer for FontSize {
  fn serialize(element: &Self) -> String {
    match element {
      Self::Css(font_size) => CssFontSize::serialize(font_size),
      Self::Decimal(number) => number.to_string(),
    }
  }
}

impl DatatypeDeserializer for FontSize {
  fn deserialize(value: &str) -> Result<Self, String> {
    if let Ok(dec_val) = value.parse::<f32>() {
      if dec_val > 0.0 {
        Ok(FontSize::Decimal(dec_val))
      } else {
        Err(format!("Value {} is invalid for the <font-size> data type", value))
      }
    } else if let Ok(font_size) = CssFontSize::deserialize(value) {
      Ok(FontSize::Css(font_size))
    } else {
      Err(format!("Value {} is invalid for the <font-size> data type", value))
    }
  }
}

#[cfg(test)]
mod font_size_tests {
  use super::*;

  #[test]
  fn deserialize_valid1() {
    let result = FontSize::deserialize("xx-small");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), FontSize::Css(CssFontSize::XxSmall));
  }

  #[test]
  fn deserialize_valid2() {
    let result = FontSize::deserialize("3");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), FontSize::Decimal(3.0))
  }

  #[test]
  fn deserialize_invalid1() {
    let result = FontSize::deserialize("Big");
    assert!(result.is_err());
  }

  #[test]
  fn deserialize_invalid2() {
    let result = FontSize::deserialize("0");
    assert!(result.is_err());
  }

  #[test]
  fn deserialize_invalid3() {
    let result = FontSize::deserialize("-4");
    assert!(result.is_err());
  }
}
