use alloc::string::String;
use musicxml_internal::{DatatypeDeserializer, DatatypeSerializer};
use musicxml_macros::{DatatypeDeserialize, DatatypeSerialize};

/// Specifies the mode similar to the mode type, but with a restricted set of values.
///
/// The different minor values are used to interpret [NuermalRoot][crate::elements::NumeralRoot] values
/// of 6 and 7 when present in a minor key. The harmonic minor value sharpens the 7 and the
/// melodic minor value sharpens both 6 and 7. If a minor mode is used without
/// qualification, either in the [Mode][crate::elements::Mode] or [NumeralMode][crate::elements::NumeralMode]
/// elements, natural minor is used.
#[derive(Debug, PartialEq, Eq, DatatypeDeserialize, DatatypeSerialize)]
pub enum NumeralMode {
  /// Numerals are interpreted relative to a harmonic minor scale with a raised 7th degree.
  #[rename("harmonic minor")]
  HarmonicMinor,
  /// Numerals are interpreted relative to a major scale.
  Major,
  /// Numerals are interpreted relative to an ascending melodic minor scale with raised 6th and 7th degrees.
  #[rename("melodic minor")]
  MelodicMinor,
  /// Numerals are interpreted relative to a natural minor scale.
  Minor,
  /// Numerals are interpreted relative to a natural minor scale.
  #[rename("natural minor")]
  NaturalMinor,
}
