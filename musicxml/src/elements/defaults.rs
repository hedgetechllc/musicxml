use super::{
  Appearance, ConcertScore, LyricFont, LyricLanguage, MusicFont, PageLayout, Scaling, StaffLayout, SystemLayout,
  WordFont,
};
use musicxml_internal::*;
use musicxml_macros::*;

/// Contents of the [Defaults] element.
#[derive(Debug, Default, PartialEq, Eq, ContentDeserialize, ContentSerialize)]
pub struct DefaultsContents {
  /// The [Scaling] element specifies the percentage of the regular scaling to use for music notation.
  pub scaling: Option<Scaling>,
  /// The [ConcertScore] element indicates whether the score is a concert score.
  pub concert_score: Option<ConcertScore>,
  /// The [PageLayout] element specifies the layout of the page.
  pub page_layout: Option<PageLayout>,
  /// The [SystemLayout] element specifies the layout of the system.
  pub system_layout: Option<SystemLayout>,
  /// The [StaffLayout] element specifies the layout of the staff.
  pub staff_layout: Vec<StaffLayout>,
  /// The [Appearance] element specifies the appearance of the score.
  pub appearance: Option<Appearance>,
  /// The [MusicFont] element specifies the music font.
  pub music_font: Option<MusicFont>,
  /// The [WordFont] element specifies the word font.
  pub word_font: Option<WordFont>,
  /// The [LyricFont] element specifies the lyric font.
  pub lyric_font: Vec<LyricFont>,
  /// The [LyricLanguage] element specifies the default language for lyrics.
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
