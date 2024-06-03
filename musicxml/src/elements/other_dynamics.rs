use crate::datatypes::SmuflGlyphName;
use musicxml_internal::*;
use musicxml_macros::*;

/// Attributes pertaining to the [OtherDynamics] element.
#[derive(Debug, Default, PartialEq, Eq, AttributeDeserialize, AttributeSerialize)]
pub struct OtherDynamicsAttributes {
  /// Indicates a particular Standard Music Font Layout (SMuFL) character using its canonical glyph name.
  /// Sometimes this is a formatting choice, and sometimes this is a refinement of the semantic meaning of an element.
  pub smufl: Option<SmuflGlyphName>,
}

/// The [OtherDynamics] element allows other dynamic marks that are not covered by combinations of the individual [Dynamics][super::Dynamics] child elements.
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
#[rename("other-dynamics")]
pub struct OtherDynamics {
  /// Element-specific attributes
  pub attributes: OtherDynamicsAttributes,
  /// Element-specific content
  pub content: String,
}
