use alloc::string::String;
use musicxml_internal::{DatatypeDeserializer, DatatypeSerializer};
use musicxml_macros::{DatatypeDeserialize, DatatypeSerialize};

/// Represents barline style information.
#[derive(Debug, PartialEq, Eq, DatatypeDeserialize, DatatypeSerialize)]
pub enum BarStyle {
  /// ![Dashed](https://hedgetechllc.github.io/musicxml/musicxml/datatypes/bar-style-dashed.png)
  Dashed,
  /// ![Dotted](https://hedgetechllc.github.io/musicxml/musicxml/datatypes/bar-style-dotted.png)
  Dotted,
  /// ![Heavy](https://hedgetechllc.github.io/musicxml/musicxml/datatypes/bar-style-heavy.png)
  Heavy,
  /// ![HeavyHeavy](https://hedgetechllc.github.io/musicxml/musicxml/datatypes/bar-style-heavy-heavy.png)
  #[rename("heavy-heavy")]
  HeavyHeavy,
  /// ![HeavyLight](https://hedgetechllc.github.io/musicxml/musicxml/datatypes/bar-style-heavy-light.png)
  #[rename("heavy-light")]
  HeavyLight,
  /// ![LightHeavy](https://hedgetechllc.github.io/musicxml/musicxml/datatypes/bar-style-light-heavy.png)
  #[rename("light-heavy")]
  LightHeavy,
  /// ![LightLight](https://hedgetechllc.github.io/musicxml/musicxml/datatypes/bar-style-light-light.png)
  #[rename("light-light")]
  LightLight,
  /// No barline appears.
  None,
  /// ![Regular](https://hedgetechllc.github.io/musicxml/musicxml/datatypes/bar-style-regular.png)
  Regular,
  /// ![Short](https://hedgetechllc.github.io/musicxml/musicxml/datatypes/bar-style-short.png)
  Short,
  /// ![Tick](https://hedgetechllc.github.io/musicxml/musicxml/datatypes/bar-style-tick.png)
  Tick,
}
