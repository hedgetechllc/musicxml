use musicxml_internal::{DatatypeDeserializer, DatatypeSerializer};
use musicxml_macros::{DatatypeDeserialize, DatatypeSerialize};

/// Indicates if the group should have common barlines.
#[derive(Debug, PartialEq, Eq, DatatypeDeserialize, DatatypeSerialize)]
pub enum GroupBarlineValue {
  /// ![Yes](https://hedgetechllc.github.io/musicxml/musicxml/datatypes/group-barline-value-yes.png)
  Yes,
  /// ![No](https://hedgetechllc.github.io/musicxml/musicxml/datatypes/group-barline-value-no.png)
  No,
  /// ![Mensurstrich](https://hedgetechllc.github.io/musicxml/musicxml/datatypes/group-barline-value-mensurstrich.png)
  #[rename("Mensurstrich")]
  Mensurstrich,
}
