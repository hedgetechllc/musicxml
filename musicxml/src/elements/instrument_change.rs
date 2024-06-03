use super::{Ensemble, InstrumentSound, Solo, VirtualInstrument};
use crate::datatypes::IdRef;
use musicxml_internal::*;
use musicxml_macros::*;

/// Attributes pertaining to the [InstrumentChange] element.
#[derive(Debug, PartialEq, Eq, AttributeDeserialize, AttributeSerialize)]
pub struct InstrumentChangeAttributes {
  /// Specifies an ID that is unique to the entire document.
  pub id: IdRef,
}

/// Contents of the [InstrumentChange] element.
#[derive(Debug, Default, PartialEq, Eq, ContentDeserialize, ContentSerialize)]
pub struct InstrumentChangeContents {
  /// The [InstrumentSound] element specifies the virtual instrument sound to use for a given [ScoreInstrument][super::ScoreInstrument].
  pub instrument_sound: Option<InstrumentSound>,
  /// The [Solo] element specifies the solo virtual instrument sound to use for a given [ScoreInstrument][super::ScoreInstrument].
  pub solo: Option<Solo>,
  /// The [Ensemble] element specifies the ensemble virtual instrument sound to use for a given [ScoreInstrument][super::ScoreInstrument].
  pub ensemble: Option<Ensemble>,
  /// The [VirtualInstrument] element specifies the virtual instrument to use for a given [ScoreInstrument][super::ScoreInstrument].
  pub virtual_instrument: Option<VirtualInstrument>,
}

/// The [InstrumentChange] element type represents a change to the virtual instrument sound for a given [ScoreInstrument][super::ScoreInstrument].
///
/// All [InstrumentChange] child elements can also be initially specified within the [ScoreInstrument][super::ScoreInstrument] element.
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
#[rename("instrument-change")]
pub struct InstrumentChange {
  /// Element-specific attributes
  pub attributes: InstrumentChangeAttributes,
  #[flatten]
  /// Element-specific content
  pub content: InstrumentChangeContents,
}
