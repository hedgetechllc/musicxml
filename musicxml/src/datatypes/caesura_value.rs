use alloc::string::String;
use musicxml_internal::{DatatypeDeserializer, DatatypeSerializer};

/// Represents the shape of the caesura sign.
#[derive(Debug, PartialEq, Eq)]
pub enum CaesuraValue {
  /// <span class="smufl">&#xE4D1;</span>
  Normal,
  /// <span class="smufl">&#xE4D2;</span>
  Thick,
  /// <span class="smufl">&#xE4D3;</span>
  Short,
  /// <span class="smufl">&#xE4D4;</span>
  Curved,
  /// <span class="smufl">&#xF42C;</span>
  Single,
  /// The [Empty][CaesuraValue::Empty] value is equivalent to the [Normal][CaesuraValue::Normal] value.
  Empty,
}

impl DatatypeSerializer for CaesuraValue {
  fn serialize(element: &Self) -> String {
    match element {
      Self::Normal => String::from("normal"),
      Self::Thick => String::from("thick"),
      Self::Short => String::from("short"),
      Self::Curved => String::from("curved"),
      Self::Single => String::from("single"),
      Self::Empty => String::new(),
    }
  }
}

impl DatatypeDeserializer for CaesuraValue {
  fn deserialize(value: &str) -> Result<Self, String> {
    Ok(match value {
      "normal" => Self::Normal,
      "thick" => Self::Thick,
      "short" => Self::Short,
      "curved" => Self::Curved,
      "single" => Self::Single,
      _ => Self::Empty,
    })
  }
}
