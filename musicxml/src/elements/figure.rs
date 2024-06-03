use super::{Extend, FigureNumber, Footnote, Level, Prefix, Suffix};
use musicxml_internal::*;
use musicxml_macros::*;

#[derive(Debug, Default, PartialEq, Eq, ContentDeserialize, ContentSerialize)]
pub struct FigureContents {
  pub prefix: Option<Prefix>,
  pub figure_number: Option<FigureNumber>,
  pub suffix: Option<Suffix>,
  pub extend: Option<Extend>,
  pub footnote: Option<Footnote>,
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
