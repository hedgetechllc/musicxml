use musicxml_internal::{DatatypeDeserializer, DatatypeSerializer};

/// Defines what type of glyph is being defined in a [Glyph][crate::elements::Glyph] element.
#[derive(Debug, PartialEq, Eq)]
pub enum GlyphType {
  /// Specifies the glyph to use when a note has a [Rest][crate::elements::Rest] element and a `type` value of quarter.
  QuarterRest,
  /// Specifies the glyph to use when a clef [Sign][crate::elements::Sign] element value is G and
  /// the [ClefOctaveChange][crate::elements::ClefOctaveChange] element value is -1.
  GClefOttavaBassa,
  /// Specifies the glyph to use when a clef [Sign][crate::elements::Sign] element value is C.
  CClef,
  /// Specifies the glyph to use when a clef [Sign][crate::elements::Sign] element value is F.
  FClef,
  /// Specifies the glyph to use when a clef [Sign][crate::elements::Sign] element value is percussion.
  PercussionClef,
  /// Specifies the glyph to use when an `octave_shift` type attribute value is up and the `size` attribute value is 8.
  OctaveShiftUp8,
  /// Specifies the glyph to use when an `octave_shift` type attribute value is down and the `size` attribute value is 8.
  OctaveShiftDown8,
  /// Specifies the glyph to use when an `octave_shift` type attribute value is continue and the `size` attribute value is 8.
  OctaveShiftContinue8,
  /// Specifies the glyph to use when an `octave_shift` type attribute value is up and the `size` attribute value is 15.
  OctaveShiftUp15,
  /// Specifies the glyph to use when an `octave_shift` type attribute value is down and the `size` attribute value is 15.
  OctaveShiftDown15,
  /// Specifies the glyph to use when an `octave_shift` type attribute value is continue and the `size` attribute value is 15.
  OctaveShiftContinue15,
  /// Specifies the glyph to use when an `octave_shift` type attribute value is up and the `size` attribute value is 22.
  OctaveShiftUp22,
  /// Specifies the glyph to use when an `octave_shift` type attribute value is down and the `size` attribute value is 22.
  OctaveShiftDown22,
  /// Specifies the glyph to use when an `octave_shift` type attribute value is continue and the `size` attribute value is 22.
  OctaveShiftContinue22,
  /// Specifies an application-specific glyph to use.
  Other(String),
}

impl DatatypeSerializer for GlyphType {
  fn serialize(element: &Self) -> String {
    match element {
      Self::QuarterRest => String::from("quarter-rest"),
      Self::GClefOttavaBassa => String::from("g-clef-ottava-bassa"),
      Self::CClef => String::from("c-clef"),
      Self::FClef => String::from("f-clef"),
      Self::PercussionClef => String::from("percussion-clef"),
      Self::OctaveShiftUp8 => String::from("octave-shift-up-8"),
      Self::OctaveShiftDown8 => String::from("octave-shift-down-8"),
      Self::OctaveShiftContinue8 => String::from("octave-shift-continue-8"),
      Self::OctaveShiftUp15 => String::from("octave-shift-up-15"),
      Self::OctaveShiftDown15 => String::from("octave-shift-down-15"),
      Self::OctaveShiftContinue15 => String::from("octave-shift-continue-15"),
      Self::OctaveShiftUp22 => String::from("octave-shift-up-22"),
      Self::OctaveShiftDown22 => String::from("octave-shift-down-22"),
      Self::OctaveShiftContinue22 => String::from("octave-shift-continue-22"),
      Self::Other(text) => text.clone(),
    }
  }
}

impl DatatypeDeserializer for GlyphType {
  fn deserialize(value: &str) -> Result<Self, String> {
    Ok(match value {
      "quarter-rest" => Self::QuarterRest,
      "g-clef-ottava-bassa" => Self::GClefOttavaBassa,
      "c-clef" => Self::CClef,
      "f-clef" => Self::FClef,
      "percussion-clef" => Self::PercussionClef,
      "octave-shift-up-8" => Self::OctaveShiftUp8,
      "octave-shift-down-8" => Self::OctaveShiftDown8,
      "octave-shift-continue-8" => Self::OctaveShiftContinue8,
      "octave-shift-up-15" => Self::OctaveShiftUp15,
      "octave-shift-down-15" => Self::OctaveShiftDown15,
      "octave-shift-continue-15" => Self::OctaveShiftContinue15,
      "octave-shift-up-22" => Self::OctaveShiftUp22,
      "octave-shift-down-22" => Self::OctaveShiftDown22,
      "octave-shift-continue-22" => Self::OctaveShiftContinue22,
      _ => Self::Other(String::from(value)),
    })
  }
}
