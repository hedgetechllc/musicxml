use super::{
  Appearance, ConcertScore, LyricFont, LyricLanguage, MusicFont, PageLayout, Scaling, StaffLayout, SystemLayout,
  WordFont,
};
use musicxml_internal::*;
use musicxml_macros::*;

#[derive(Debug, Default, PartialEq, Eq, ContentDeserialize, ContentSerialize)]
pub struct DefaultsContents {
  pub scaling: Option<Scaling>,
  pub concert_score: Option<ConcertScore>,
  pub page_layout: Option<PageLayout>,
  pub system_layout: Option<SystemLayout>,
  pub staff_layout: Vec<StaffLayout>,
  pub appearance: Option<Appearance>,
  pub music_font: Option<MusicFont>,
  pub word_font: Option<WordFont>,
  pub lyric_font: Vec<LyricFont>,
  pub lyric_language: Vec<LyricLanguage>,
}

/// The [Defaults] element specifies score-wide defaults for scaling, whether or not the file is a concert score,
/// layout, and default values for the music font, word font, lyric font, and lyric language.
///
/// Except for the [ConcertScore] element, if any defaults are missing, the choice of what to use is determined by the application.
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
pub struct Defaults {
  /// Element-specific attributes
  pub attributes: (),
  #[flatten]
  /// Element-specific content
  pub content: DefaultsContents,
}
