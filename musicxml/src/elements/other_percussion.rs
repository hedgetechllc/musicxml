use crate::datatypes::SmuflGlyphName;
use alloc::{string::String, vec::Vec};
use musicxml_internal::*;
use musicxml_macros::*;

/// Attributes pertaining to the [OtherPercussion] element.
#[derive(Debug, Default, PartialEq, Eq, AttributeDeserialize, AttributeSerialize)]
pub struct OtherPercussionAttributes {
  /// Indicates a particular Standard Music Font Layout (SMuFL) character using its canonical glyph name.
  /// Sometimes this is a formatting choice, and sometimes this is a refinement of the semantic meaning of an element.
  pub smufl: Option<SmuflGlyphName>,
}

/// The [OtherPercussion] element represents percussion pictograms not defined elsewhere.
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
#[rename("other-percussion")]
pub struct OtherPercussion {
  /// Element-specific attributes
  pub attributes: OtherPercussionAttributes,
  /// Element-specific content
  pub content: String,
}
