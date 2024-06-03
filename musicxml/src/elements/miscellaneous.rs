use super::MiscellaneousField;
use musicxml_internal::*;
use musicxml_macros::*;

#[derive(Debug, Default, PartialEq, Eq, ContentDeserialize, ContentSerialize)]
pub struct MiscellaneousContents {
  pub miscellaneous_field: Vec<MiscellaneousField>,
}

/// If a program has other metadata not yet supported in the MusicXML format, it can go in the [Miscellaneous] element.
/// 
/// The [Miscellaneous] element puts each separate part of metadata into its own [MiscellaneousField] element.
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
pub struct Miscellaneous {
  /// Element-specific attributes
  pub attributes: (),
  #[flatten]
  /// Element-specific content
  pub content: MiscellaneousContents,
}
