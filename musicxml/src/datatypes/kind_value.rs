use musicxml_internal::{DatatypeDeserializer, DatatypeSerializer};
use musicxml_macros::{DatatypeDeserialize, DatatypeSerialize};

/// Indicates the type of chord.
/// 
/// [Degree][crate::elements::Degree] elements can then add, subtract, or alter from these starting points.
/// 
/// The 11th and 13th values are usually used as a basis for alteration.
#[derive(Debug, PartialEq, Eq, DatatypeDeserialize, DatatypeSerialize)]
pub enum KindValue {
  /// Triad: major third, augmented fifth.
  Augmented,
  /// Seventh: augmented triad, minor seventh.
  #[rename("augmented-seventh")]
  AugmentedSeventh,
  /// Triad: minor third, diminished fifth.
  Diminished,
  /// Seventh: diminished triad, diminished seventh.
  #[rename("diminished-seventh")]
  DiminishedSeventh,
  /// Seventh: major triad, minor seventh.
  Dominant,
  /// 11th: dominant-ninth, perfect 11th.
  #[rename("dominant-11th")]
  Dominant11th,
  /// 13th: dominant-11th, major 13th.
  #[rename("dominant-13th")]
  Dominant13th,
  /// Ninth: dominant, major ninth.
  #[rename("dominant-ninth")]
  DominantNinth,
  /// Functional French sixth.
  #[rename("French")]
  French,
  /// Functional German sixth.
  #[rename("German")]
  German,
  /// Seventh: diminished triad, minor seventh.
  #[rename("half-diminished")]
  HalfDiminished,
  /// Functional Italian sixth.
  #[rename("Italian")]
  Italian,
  /// Triad: major third, perfect fifth.
  Major,
  /// 11th: major-ninth, perfect 11th.
  #[rename("major-11th")]
  Major11th,
  /// 13th: major-11th, major 13th.
  #[rename("major-13th")]
  Major13th,
  /// Seventh: minor triad, major seventh.
  #[rename("major-minor")]
  MajorMinor,
  /// Ninth: major-seventh, major ninth.
  #[rename("major-ninth")]
  MajorNinth,
  /// Seventh: major triad, major seventh.
  #[rename("major-seventh")]
  MajorSeventh,
  /// Sixth: major triad, added sixth.
  #[rename("major-sixth")]
  MajorSixth,
  /// Triad: minor third, perfect fifth.  
  Minor,
  /// 11th: minor-ninth, perfect 11th.
  #[rename("minor-11th")]
  Minor11th,
  /// 13th: minor-11th, major 13th.
  #[rename("minor-13th")]
  Minor13th,
  /// Ninth: minor-seventh, major ninth.
  #[rename("minor-ninth")]
  MinorNinth,
  /// Seventh: minor triad, minor seventh.
  #[rename("minor-seventh")]
  MinorSeventh,
  /// Sixth: minor triad, added sixth.
  #[rename("minor-sixth")]
  MinorSixth,
  /// Functional Neapolitan sixth.
  #[rename("Neapolitan")]
  Neapolitan,
  /// Used to explicitly encode the absence of chords or functional harmony.
  /// 
  /// In this case, the [Root][crate::elements::Root], [Numeral][crate::elements::Numeral],
  /// or [Function][crate::elements::Function] element has no meaning.
  /// 
  /// When using the [Root][crate::elements::Root] or [Numeral][crate::elements::Numeral] element,
  /// the `root_step` or `numeral_step` attribute should be set to the empty string to keep the
  /// root or numeral from being displayed.
  None,
  /// Used when the harmony is entirely composed of add elements.
  Other,
  /// Pedal-point bass.
  Pedal,
  /// Perfect fifth.
  Power,
  /// Suspended: perfect fourth, perfect fifth.
  #[rename("suspended-fourth")]
  SuspendedFourth,
  /// Suspended: major second, perfect fifth.
  #[rename("suspended-second")]
  SuspendedSecond,
  /// Augmented fourth, augmented sixth, augmented ninth.
  #[rename("Tristan")]
  Tristan,
}
