use musicxml_internal::*;
use musicxml_macros::*;

/// The [CreditType] element indicates the purpose behind a credit.
/// 
/// Multiple types of data may be combined in a single credit, so multiple elements may be used. Standard values include:
/// 
/// - page number
/// - title
/// - subtitle
/// - composer
/// - arranger
/// - lyricist
/// - rights
/// - part name
/// 
/// Other values may also be used.
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
#[rename("credit-type")]
pub struct CreditType {
  /// Element-specific attributes
  pub attributes: (),
  /// Element-specific content
  pub content: String,
}
