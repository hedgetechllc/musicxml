use musicxml_internal::{DatatypeDeserializer, DatatypeSerializer};
use musicxml_macros::{DatatypeDeserialize, DatatypeSerialize};

/// Represents barline style information.
#[derive(Debug, PartialEq, Eq, DatatypeDeserialize, DatatypeSerialize)]
pub enum BarStyle {
  /// ![Dashed](bar-style-dashed.png)
  Dashed,
  /// ![Dotted](bar-style-dotted.png)
  Dotted,
  /// ![Heavy](bar-style-heavy.png)
  Heavy,
  /// ![HeavyHeavy](bar-style-heavy-heavy.png)
  #[rename("heavy-heavy")]
  HeavyHeavy,
  /// ![HeavyLight](bar-style-heavy-light.png)
  #[rename("heavy-light")]
  HeavyLight,
  /// ![LightHeavy](bar-style-light-heavy.png)
  #[rename("light-heavy")]
  LightHeavy,
  /// ![LightLight](bar-style-light-light.png)
  #[rename("light-light")]
  LightLight,
  /// No barline appears.
  None,
  /// ![Regular](bar-style-regular.png)
  Regular,
  /// ![Short](bar-style-short.png)
  Short,
  /// ![Tick](bar-style-tick.png)
  Tick,
}
