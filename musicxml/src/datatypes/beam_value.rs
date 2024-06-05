use musicxml_internal::{DatatypeDeserializer, DatatypeSerializer};
use musicxml_macros::{DatatypeDeserialize, DatatypeSerialize};

/// Represents the type of beam associated with each of 8 beam levels (up to 1024th notes) available for each note.
#[derive(Debug, PartialEq, Eq, DatatypeDeserialize, DatatypeSerialize)]
pub enum BeamValue {
  /// ![BackwardHook](https://hedgetechllc.github.io/musicxml/musicxml/datatypes/beam-value-backward-hook.png)
  #[rename("backward hook")]
  BackwardHook,
  /// ![Begin](https://hedgetechllc.github.io/musicxml/musicxml/datatypes/beam-value-begin.png)
  Begin,
  /// ![Continue](https://hedgetechllc.github.io/musicxml/musicxml/datatypes/beam-value-continue.png)
  Continue,
  /// ![End](https://hedgetechllc.github.io/musicxml/musicxml/datatypes/beam-value-end.png)
  End,
  /// ![ForwardHook](https://hedgetechllc.github.io/musicxml/musicxml/datatypes/beam-value-forward-hook.png)
  #[rename("forward hook")]
  ForwardHook,
}
