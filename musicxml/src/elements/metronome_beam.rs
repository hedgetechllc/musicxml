use crate::datatypes::{BeamLevel, BeamValue};
use musicxml_internal::*;
use musicxml_macros::*;

/// Attributes pertaining to the [MetronomeBeam] element.
#[derive(Debug, Default, PartialEq, Eq, AttributeDeserialize, AttributeSerialize)]
pub struct MetronomeBeamAttributes {
  /// Indicates eighth note through 1024th note beams using number values 1 thru 8 respectively. The default value is 1.
  pub number: Option<BeamLevel>,
}

/// The [MetronomeBeam] element works like the [Beam][super::Beam] element in defining metric relationships,
/// but does not include all the attributes available in the [Beam][super::Beam] element.
///
/// ![MetronomeBeam](metronome-beam.png)
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
#[rename("metronome-beam")]
pub struct MetronomeBeam {
  /// Element-specific attributes
  pub attributes: MetronomeBeamAttributes,
  /// Element-specific content
  pub content: BeamValue,
}
