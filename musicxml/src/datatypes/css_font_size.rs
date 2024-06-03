use musicxml_internal::{DatatypeDeserializer, DatatypeSerializer};
use musicxml_macros::{DatatypeDeserialize, DatatypeSerialize};

/// Includes the [CSS font sizes](https://www.w3.org/TR/css-fonts-3/#font-size-prop) used
/// as an alternative to a numeric point size.
///
/// In CSS, these refer to an entry in a table of font sizes computed and kept by the user agent.
/// The scaling is relative to the reference value of [Medium][CssFontSize::Medium].
#[derive(Debug, PartialEq, Eq, DatatypeDeserialize, DatatypeSerialize)]
pub enum CssFontSize {
  /// Scaling factor guideline: 3/5 of [Medium][CssFontSize::Medium].
  #[rename("xx-small")]
  XxSmall,
  /// Scaling factor guideline: 3/4 of [Medium][CssFontSize::Medium].
  #[rename("x-small")]
  XSmall,
  /// Scaling factor guideline: 8/9 of [Medium][CssFontSize::Medium].
  Small,
  /// Scaling factor guideline: equal to [Medium][CssFontSize::Medium].
  Medium,
  /// Scaling factor guideline: 6/5 of [Medium][CssFontSize::Medium].
  Large,
  /// Scaling factor guideline: 3/2 of [Medium][CssFontSize::Medium].
  #[rename("x-large")]
  XLarge,
  /// Scaling factor guideline: 2/1 of [Medium][CssFontSize::Medium].
  #[rename("xx-large")]
  XxLarge,
}
