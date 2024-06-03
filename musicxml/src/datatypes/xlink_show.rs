use musicxml_internal::{DatatypeDeserializer, DatatypeSerializer};
use musicxml_macros::{DatatypeDeserialize, DatatypeSerialize};

/// See the definition in the [XML Linking Language recommendation](https://www.w3.org/TR/xlink11/#link-behaviors).
#[derive(Debug, PartialEq, Eq, DatatypeDeserialize, DatatypeSerialize)]
pub enum XlinkShow {
  /// An application traversing to the ending resource should load it in a new window,
  /// frame, pane, or other relevant presentation context.
  New,
  /// An application traversing to the ending resource should load the resource in the same window,
  /// frame, pane, or other relevant presentation context in which the starting resource was loaded.
  /// This is the default value.
  Replace,
  /// An application traversing to the ending resource should load its presentation in
  /// place of the presentation of the starting resource.
  Embed,
  /// The behavior of an application traversing to the ending resource is unconstrained
  /// by this specification. The application should look for other markup present in the link
  /// to determine the appropriate behavior.
  Other,
  /// The behavior of an application traversing to the ending resource is unconstrained
  /// by this specification. No other markup is present to help the application determine
  /// the appropriate behavior.
  None,
}
