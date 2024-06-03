use super::{Extend, FigureNumber, Footnote, Level, Prefix, Suffix};
use musicxml_internal::*;
use musicxml_macros::*;

/// Contents of the [Figure] element.
#[derive(Debug, Default, PartialEq, Eq, ContentDeserialize, ContentSerialize)]
pub struct FigureContents {
  /// The [Prefix] element is used to specify the prefix of a figured bass symbol.
  pub prefix: Option<Prefix>,
  /// The [FigureNumber] element is used to specify the figure number of a figured bass symbol.
  pub figure_number: Option<FigureNumber>,
  /// The [Suffix] element is used to specify the suffix of a figured bass symbol.
  pub suffix: Option<Suffix>,
  /// The [Extend] element is used to specify the extension of a figured bass symbol.
  pub extend: Option<Extend>,
  /// The [Footnote] element is used to specify a footnote for a figured bass symbol.
  pub footnote: Option<Footnote>,
  /// The [Level] element is used to specify the level of a figured bass symbol.
  pub level: Option<Level>,
}

/// The [Figure] element represents a single figure within a [FiguredBass][super::FiguredBass] element.
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
pub struct Figure {
  /// Element-specific attributes
  pub attributes: (),
  #[flatten]
  /// Element-specific content
  pub content: FigureContents,
}
