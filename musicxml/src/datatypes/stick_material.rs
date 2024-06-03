use musicxml_internal::{DatatypeDeserializer, DatatypeSerializer};
use musicxml_macros::{DatatypeDeserialize, DatatypeSerialize};

/// Represents the material being displayed in a stick pictogram.
#[derive(Debug, PartialEq, Eq, DatatypeDeserialize, DatatypeSerialize)]
pub enum StickMaterial {
  /// <span class="smufl">&#xE778;</span>
  Hard,
  /// <span class="smufl">&#xE774;</span>
  Medium,
  /// <span class="smufl">&#xE77C;</span>
  Shaded,
  /// <span class="smufl">&#xE770;</span>
  Soft,
  /// <span class="smufl">&#xE7C7;</span>
  X,
}
