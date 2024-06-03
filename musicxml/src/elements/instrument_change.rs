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

#[derive(Debug, Default, PartialEq, Eq, ContentDeserialize, ContentSerialize)]
pub struct InstrumentChangeContents {
  pub instrument_sound: Option<InstrumentSound>,
  pub solo: Option<Solo>,
  pub ensemble: Option<Ensemble>,
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
