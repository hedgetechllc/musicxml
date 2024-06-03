use super::{
  Arrow, Bend, BrassBend, DoubleTongue, DownBow, Fingering, Fingernails, Flip, Fret, Golpe, HalfMuted, HammerOn,
  Handbell, HarmonMute, Harmonic, Heel, Hole, Open, OpenString, OtherTechnical, Pluck, PullOff, Smear, SnapPizzicato,
  Stopped, StringNumber, Tap, ThumbPosition, Toe, TripleTongue, UpBow,
};
use crate::datatypes::Id;
use musicxml_internal::*;
use musicxml_macros::*;

/// Attributes pertaining to the [Technical] element.
#[derive(Debug, Default, PartialEq, Eq, AttributeDeserialize, AttributeSerialize)]
pub struct TechnicalAttributes {
  /// Specifies an ID that is unique to the entire document.
  pub id: Option<Id>,
}

#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
pub enum TechnicalContents {
  #[rename("up-bow")]
  UpBow(UpBow),
  #[rename("down-bow")]
  DownBow(DownBow),
  Harmonic(Harmonic),
  #[rename("open-string")]
  OpenString(OpenString),
  #[rename("thumb-position")]
  ThumbPosition(ThumbPosition),
  Fingering(Fingering),
  Pluck(Pluck),
  #[rename("double-tongue")]
  DoubleTongue(DoubleTongue),
  #[rename("triple-tongue")]
  TripleTongue(TripleTongue),
  Stopped(Stopped),
  #[rename("snap-pizzicato")]
  SnapPizzicato(SnapPizzicato),
  Fret(Fret),
  #[rename("string")]
  StringNumber(StringNumber),
  #[rename("hammer-on")]
  HammerOn(HammerOn),
  #[rename("pull-off")]
  PullOff(PullOff),
  Bend(Bend),
  Tap(Tap),
  Heel(Heel),
  Toe(Toe),
  Fingernails(Fingernails),
  Hole(Hole),
  Arrow(Arrow),
  Handbell(Handbell),
  #[rename("brass-bend")]
  BrassBend(BrassBend),
  Flip(Flip),
  Smear(Smear),
  Open(Open),
  #[rename("half-muted")]
  HalfMuted(HalfMuted),
  #[rename("harmon-mute")]
  HarmonMute(HarmonMute),
  Golpe(Golpe),
  #[rename("other-technical")]
  OtherTechnical(OtherTechnical),
}

/// The [Technical] element groups together technical indications that give performance information for specific instruments.
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
pub struct Technical {
  /// Element-specific attributes
  pub attributes: TechnicalAttributes,
  /// Element-specific content
  pub content: Vec<TechnicalContents>,
}
