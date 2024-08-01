use crate::datatypes::{GlyphType, SmuflGlyphName};
use alloc::{string::String, vec::Vec};
use musicxml_internal::*;
use musicxml_macros::*;

/// Attributes pertaining to the [Glyph] element.
#[derive(Debug, PartialEq, Eq, AttributeDeserialize, AttributeSerialize)]
pub struct GlyphAttributes {
  /// 	The type of glyph that is being defined.
  pub r#type: GlyphType,
}

/// The [Glyph] element represents what Standard Music Font Layout (SMuFL) glyph should be used for different variations of symbols that are semantically identical.
///
/// The `type` attribute specifies what type of glyph is being defined. The element value specifies what SMuFL canonical glyph name to use,
/// including recommended stylistic alternates.
///
/// The SMuFL canonical glyph name should match the type. For instance, a type of quarter-rest would use values "restQuarter," "restQuarterOld," or
/// "restQuarterZ". A type of g-clef-ottava-bassa would use values "gClef8vb," "gClef8vbOld," or "gClef8vbCClef". A type of octave-shift-up-8 would use values
/// "ottava," "ottavaBassa," "ottavaBassaBa," "ottavaBassaVb," or "octaveBassa".
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
pub struct Glyph {
  /// Element-specific attributes
  pub attributes: GlyphAttributes,
  /// Element-specific content
  pub content: SmuflGlyphName,
}
