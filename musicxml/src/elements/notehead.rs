use crate::datatypes::{Color, FontFamily, FontSize, FontStyle, FontWeight, NoteheadValue, SmuflGlyphName, YesNo};
use alloc::{string::String, vec::Vec};
use musicxml_internal::*;
use musicxml_macros::*;

/// Attributes pertaining to the [Notehead] element.
#[derive(Debug, Default, PartialEq, Eq, AttributeDeserialize, AttributeSerialize)]
pub struct NoteheadAttributes {
  /// Indicates the color of an element.
  pub color: Option<Color>,
  /// Changes the appearance of enclosed shapes from the default of hollow for half notes and longer, and filled otherwise.
  pub filled: Option<YesNo>,
  /// A comma-separated list of font names.
  pub font_family: Option<FontFamily>,
  /// One of the CSS sizes or a numeric point size.
  pub font_size: Option<FontSize>,
  /// Normal or italic style.
  pub font_style: Option<FontStyle>,
  /// Normal or bold weight.
  pub font_weight: Option<FontWeight>,
  /// Specifies whether or not parentheses are put around a symbol for an editorial indication. If not specified, it is left to application defaults.
  pub parentheses: Option<YesNo>,
  /// Indicates a particular Standard Music Font Layout (SMuFL) character using its canonical glyph name.
  /// Sometimes this is a formatting choice, and sometimes this is a refinement of the semantic meaning of an element.
  pub smufl: Option<SmuflGlyphName>,
}

/// The [Notehead] element indicates shapes other than the open and closed ovals associated with note durations.
///
/// The `smufl` attribute can be used to specify a particular notehead, allowing application interoperability without requiring every
/// Standard Music Font Layout (SMuFL) glyph to have a MusicXML element equivalent. This attribute can be used either with the other value,
/// or to refine a specific notehead value such as "cluster".
///
/// Noteheads in the SMuFL [Note name noteheads](https://www.w3.org/2021/03/smufl14/tables/note-name-noteheads.html) and
/// [Note name noteheads supplement](https://www.w3.org/2021/03/smufl14/tables/note-name-noteheads-supplement.html) ranges
/// (U+E150–U+E1AF and U+EEE0–U+EEFF) should not use the `smufl` attribute or the `other` value, but instead use
/// the [NoteheadText][super::NoteheadText] element.
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
pub struct Notehead {
  /// Element-specific attributes
  pub attributes: NoteheadAttributes,
  /// Element-specific content
  pub content: NoteheadValue,
}
