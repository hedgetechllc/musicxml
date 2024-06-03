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

/// The [TechnicalContents] element specifies all possible options available for use in a [Technical] element.
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
pub enum TechnicalContents {
  /// The [UpBow] element represents the symbol for up-bow.
  #[rename("up-bow")]
  UpBow(UpBow),
  /// The [DownBow] element represents the symbol for down-bow.
  #[rename("down-bow")]
  DownBow(DownBow),
  /// The [Harmonic] element represents a harmonic.
  Harmonic(Harmonic),
  /// The [OpenString] element represents an open string.
  #[rename("open-string")]
  OpenString(OpenString),
  /// The [ThumbPosition] element represents the thumb position.
  #[rename("thumb-position")]
  ThumbPosition(ThumbPosition),
  /// The [Fingering] element represents a fingering.
  Fingering(Fingering),
  /// The [Pluck] element represents the symbol for plucking.
  Pluck(Pluck),
  /// The [DoubleTongue] element represents the symbol for double-tongue.
  #[rename("double-tongue")]
  DoubleTongue(DoubleTongue),
  /// The [TripleTongue] element represents the symbol for triple-tongue.
  #[rename("triple-tongue")]
  TripleTongue(TripleTongue),
  /// The [Stopped] element represents the symbol for stopped.
  Stopped(Stopped),
  /// The [SnapPizzicato] element represents the symbol for snap pizzicato.
  #[rename("snap-pizzicato")]
  SnapPizzicato(SnapPizzicato),
  /// The [Fret] element represents a fret number.
  Fret(Fret),
  /// The [StringNumber] element represents a string number.
  #[rename("string")]
  StringNumber(StringNumber),
  /// The [HammerOn] element represents the symbol for hammer-on.
  #[rename("hammer-on")]
  HammerOn(HammerOn),
  /// The [PullOff] element represents the symbol for pull-off.
  #[rename("pull-off")]
  PullOff(PullOff),
  /// The [Bend] element represents the symbol for bend.
  Bend(Bend),
  /// The [Tap] element represents the symbol for tap.
  Tap(Tap),
  /// The [Heel] element represents the symbol for heel.
  Heel(Heel),
  /// The [Toe] element represents the symbol for toe.
  Toe(Toe),
  /// The [Fingernails] element represents the symbol for fingernails.
  Fingernails(Fingernails),
  /// The [Hole] element represents the symbol for hole.
  Hole(Hole),
  /// The [Arrow] element represents the symbol for arrow.
  Arrow(Arrow),
  /// The [Handbell] element represents the symbol for handbell.
  Handbell(Handbell),
  /// The [BrassBend] element represents the symbol for brass bend.
  #[rename("brass-bend")]
  BrassBend(BrassBend),
  /// The [Flip] element represents the symbol for flip.
  Flip(Flip),
  /// The [Smear] element represents the symbol for smear.
  Smear(Smear),
  /// The [Open] element represents the symbol for open.
  Open(Open),
  /// The [HalfMuted] element represents the symbol for half-muted.
  #[rename("half-muted")]
  HalfMuted(HalfMuted),
  /// The [HarmonMute] element represents the symbol for harmon mute.
  #[rename("harmon-mute")]
  HarmonMute(HarmonMute),
  /// The [Golpe] element represents the symbol for golpe.
  Golpe(Golpe),
  /// The [OtherTechnical] element represents a technical indication not covered by other elements.
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
