use alloc::string::String;
use musicxml_internal::{DatatypeDeserializer, DatatypeSerializer};

/// Represents the shape of the fermata sign.
#[derive(Debug, PartialEq, Eq)]
pub enum FermataShape {
  /// <span class="smufl">&#xE4C0;</span>
  Normal,
  /// <span class="smufl">&#xE4C4;</span>
  Angled,
  /// <span class="smufl">&#xE4C6;</span>
  Square,
  /// <span class="smufl">&#xE4C2;</span>
  DoubleAngled,
  /// <span class="smufl">&#xE4C8;</span>
  DoubleSquare,
  /// <span class="smufl">&#xE4CA;</span>
  DoubleDot,
  /// <span class="smufl">&#xE4CC;</span>
  HalfCurve,
  /// <span class="smufl">&#xE4D6;</span>
  Curlew,
  /// The [Empty][FermataShape::Empty] value is equivalent to the [Normal][FermataShape::Normal] value.
  Empty,
}

impl DatatypeSerializer for FermataShape {
  fn serialize(element: &Self) -> String {
    match element {
      Self::Normal => String::from("normal"),
      Self::Angled => String::from("angled"),
      Self::Square => String::from("square"),
      Self::DoubleAngled => String::from("double-angled"),
      Self::DoubleSquare => String::from("double-square"),
      Self::DoubleDot => String::from("double-dot"),
      Self::HalfCurve => String::from("half-curve"),
      Self::Curlew => String::from("curlew"),
      Self::Empty => String::new(),
    }
  }
}

impl DatatypeDeserializer for FermataShape {
  fn deserialize(value: &str) -> Result<Self, String> {
    Ok(match value {
      "normal" => Self::Normal,
      "angled" => Self::Angled,
      "square" => Self::Square,
      "double-angled" => Self::DoubleAngled,
      "double-square" => Self::DoubleSquare,
      "double-dot" => Self::DoubleDot,
      "half-curve" => Self::HalfCurve,
      "curlew" => Self::Curlew,
      _ => Self::Empty,
    })
  }
}
