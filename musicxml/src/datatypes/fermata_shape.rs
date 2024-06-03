use musicxml_internal::{DatatypeDeserializer, DatatypeSerializer};
use musicxml_macros::{DatatypeDeserialize, DatatypeSerialize};

/// Represents the shape of the fermata sign.
#[derive(Debug, PartialEq, Eq, DatatypeDeserialize, DatatypeSerialize)]
pub enum FermataShape {
  /// <span class="smufl">&#xE4C0;</span>
  Normal,
  /// <span class="smufl">&#xE4C4;</span>
  Angled,
  /// <span class="smufl">&#xE4C6;</span>
  Square,
  /// <span class="smufl">&#xE4C2;</span>
  #[rename("double-angled")]
  DoubleAngled,
  /// <span class="smufl">&#xE4C8;</span>
  #[rename("double-square")]
  DoubleSquare,
  /// <span class="smufl">&#xE4CA;</span>
  #[rename("double-dot")]
  DoubleDot,
  /// <span class="smufl">&#xE4CC;</span>
  #[rename("half-curve")]
  HalfCurve,
  /// <span class="smufl">&#xE4D6;</span>
  Curlew,
  /// The [Empty][FermataShape::Empty] value is equivalent to the [Normal][FermataShape::Normal] value.
  Empty,
}
