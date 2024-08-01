use crate::datatypes::{NmToken, Token, XmlLang};
use alloc::{string::String, vec::Vec};
use musicxml_internal::*;
use musicxml_macros::*;

/// Attributes pertaining to the [LyricLanguage] element.
#[derive(Debug, PartialEq, Eq, AttributeDeserialize, AttributeSerialize)]
pub struct LyricLanguageAttributes {
  /// Specifies the language used in the element content. It is Italian ("it") if not specified.
  pub xml_lang: XmlLang,
  /// The lyric name for which this is the default, corresponding to the `name` attribute in the [Lyric][super::Lyric] element.
  pub name: Option<Token>,
  /// The lyric number for which this is the default, corresponding to the `number` attribute in the [Lyric][super::Lyric] element.
  pub number: Option<NmToken>,
}

/// The [LyricLanguage] element specifies the default language for a particular name and number of lyric.
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
#[rename("lyric-language")]
pub struct LyricLanguage {
  /// Element-specific attributes
  pub attributes: LyricLanguageAttributes,
  /// Element-specific content
  pub content: (),
}
