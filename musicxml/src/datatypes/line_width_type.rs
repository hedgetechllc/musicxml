use alloc::string::String;
use musicxml_internal::{DatatypeDeserializer, DatatypeSerializer};

/// Defines what type of line is being defined in a [LineWidth][crate::elements::LineWidth] element.
#[derive(Debug, PartialEq, Eq)]
pub enum LineWidthType {
  /// A beam line.
  Beam,
  /// A bracket line.
  Bracket,
  /// A dashed line.
  Dashes,
  /// An enclosure line.
  Enclosure,
  /// An ending line.
  Ending,
  /// An extend line.
  Extend,
  /// A heavy barline.
  HeavyBarline,
  /// A leger line.
  Leger,
  /// A light barline.
  LightBarline,
  /// An octave shift line.
  OctaveShift,
  /// A pedal line.
  Pedal,
  /// A slur middle line.
  SlurMiddle,
  /// A slur tip line.
  SlurTip,
  /// A staff line.
  Staff,
  /// A stem line.
  Stem,
  /// A tie middle line.
  TieMiddle,
  /// A tie tip line.
  TieTip,
  /// A tuplet bracket line.
  TupletBracket,
  /// A wedge line.
  Wedge,
  /// Any other type of line.
  Other(String),
}

impl DatatypeSerializer for LineWidthType {
  fn serialize(element: &Self) -> String {
    match element {
      Self::Beam => String::from("beam"),
      Self::Bracket => String::from("bracket"),
      Self::Dashes => String::from("dashes"),
      Self::Enclosure => String::from("enclosure"),
      Self::Ending => String::from("ending"),
      Self::Extend => String::from("extend"),
      Self::HeavyBarline => String::from("heavy barline"),
      Self::Leger => String::from("leger"),
      Self::LightBarline => String::from("light barline"),
      Self::OctaveShift => String::from("octave shift"),
      Self::Pedal => String::from("pedal"),
      Self::SlurMiddle => String::from("slur middle"),
      Self::SlurTip => String::from("slur tip"),
      Self::Staff => String::from("staff"),
      Self::Stem => String::from("stem"),
      Self::TieMiddle => String::from("tie middle"),
      Self::TieTip => String::from("tie tip"),
      Self::TupletBracket => String::from("tuplet bracket"),
      Self::Wedge => String::from("wedge"),
      Self::Other(text) => text.clone(),
    }
  }
}

impl DatatypeDeserializer for LineWidthType {
  fn deserialize(value: &str) -> Result<Self, String> {
    Ok(match value {
      "beam" => Self::Beam,
      "bracket" => Self::Bracket,
      "dashes" => Self::Dashes,
      "enclosure" => Self::Enclosure,
      "ending" => Self::Ending,
      "extend" => Self::Extend,
      "heavy barline" => Self::HeavyBarline,
      "leger" => Self::Leger,
      "light barline" => Self::LightBarline,
      "octave shift" => Self::OctaveShift,
      "pedal" => Self::Pedal,
      "slur middle" => Self::SlurMiddle,
      "slur tip" => Self::SlurTip,
      "staff" => Self::Staff,
      "stem" => Self::Stem,
      "tie middle" => Self::TieMiddle,
      "tie tip" => Self::TieTip,
      "tuplet bracket" => Self::TupletBracket,
      "wedge" => Self::Wedge,
      _ => Self::Other(String::from(value)),
    })
  }
}
