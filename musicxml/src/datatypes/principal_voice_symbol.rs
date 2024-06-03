use musicxml_internal::{DatatypeDeserializer, DatatypeSerializer};
use musicxml_macros::{DatatypeDeserialize, DatatypeSerialize};

/// Represents the type of symbol used to indicate a principal or secondary voice.
#[derive(Debug, PartialEq, Eq, DatatypeDeserialize, DatatypeSerialize)]
pub enum PrincipalVoiceSymbol {
  /// <span class="smufl">&#xE860;</span>
  #[rename("Hauptstimme")]
  Hauptstimme,
  /// <span class="smufl">&#xE861;</span>
  #[rename("Nebenstimme")]
  Nebenstimme,
  /// <span class="smufl">&#xE862;</span> if the type is "start",
  ///  
  /// <span class="smufl">&#xE863;</span> if the type is "stop".
  Plain,
  /// Used for analysis markup when the [PrincipalVoice][crate::elements::PrincipalVoice] element does not have a corresponding appearance in the score.
  None,
}
