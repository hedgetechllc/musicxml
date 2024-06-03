use musicxml_internal::{DatatypeDeserializer, DatatypeSerializer};
use musicxml_macros::{DatatypeDeserialize, DatatypeSerialize};

/// See the definition in the [XML Linking Language recommendation](https://www.w3.org/TR/xlink11/#link-behaviors).
#[derive(Debug, PartialEq, Eq, DatatypeDeserialize, DatatypeSerialize)]
pub enum XlinkActuate {
  /// An application should traverse from the starting resource to the ending resource only on a
  /// post-loading event triggered for the purpose of traversal. This is the default value.
  #[rename("onRequest")]
  OnRequest,
  /// An application should traverse to the ending resource immediately on loading the starting resource.
  #[rename("onLoad")]
  OnLoad,
  /// The behavior of an application traversing to the ending resource is unconstrained by this specification.
  /// The application should look for other markup present in the link to determine the appropriate behavior.
  Other,
  /// The behavior of an application traversing to the ending resource is unconstrained by this specification.
  /// No other markup is present to help the application determine the appropriate behavior.
  None,
}
