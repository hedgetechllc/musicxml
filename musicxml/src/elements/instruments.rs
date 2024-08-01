use crate::datatypes::NonNegativeInteger;
use alloc::string::String;
use musicxml_internal::*;
use musicxml_macros::*;

/// The [Instruments] element is only used if more than one instrument is represented in the part.
///
/// For example, oboe I and II where they play together most of the time. If absent, a value of 1 is assumed.
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
pub struct Instruments {
  /// Element-specific attributes
  pub attributes: (),
  /// Element-specific content
  pub content: NonNegativeInteger,
}
