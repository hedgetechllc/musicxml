use super::{
  Ff, Fff, Ffff, Fffff, Ffffff, Fp, Fz, Mf, Mp, OtherDynamics, Pf, Pp, Ppp, Pppp, Ppppp, Pppppp, Rf, Rfz, Sf, Sffz,
  Sfp, Sfpp, Sfz, Sfzp, F, N, P,
};
use crate::datatypes::{
  AboveBelow, Color, EnclosureShape, FontFamily, FontSize, FontStyle, FontWeight, Id, LeftCenterRight, NumberOfLines,
  Tenths, Valign,
};
use musicxml_internal::*;
use musicxml_macros::*;

/// Attributes pertaining to the [Dynamics] element.
#[derive(Debug, Default, PartialEq, Eq, AttributeDeserialize, AttributeSerialize)]
pub struct DynamicsAttributes {
  /// Indicates the color of an element.
  pub color: Option<Color>,
  /// Changes the computation of the default horizontal position.
  /// The origin is changed relative to the left-hand side of the note or the musical position within the bar.
  /// Positive x is right and negative x is left.
  ///
  /// This attribute provides higher-resolution positioning data than the [Offset][super::Offset] element.
  /// Applications reading a MusicXML file that can understand both features should generally rely on this attribute for its greater accuracy.
  pub default_x: Option<Tenths>,
  /// Changes the computation of the default vertical position.
  /// The origin is changed relative to the top line of the staff. Positive y is up and negative y is down.
  ///
  /// This attribute provides higher-resolution positioning data than the `placement` attribute.
  /// Applications reading a MusicXML file that can understand both attributes should generally rely on this attribute for its greater accuracy.
  pub default_y: Option<Tenths>,
  /// Formatting of an enclosure around text or symbols.
  pub enclosure: Option<EnclosureShape>,
  /// A comma-separated list of font names.
  pub font_family: Option<FontFamily>,
  /// One of the CSS sizes or a numeric point size.
  pub font_size: Option<FontSize>,
  /// Normal or italic style.
  pub font_style: Option<FontStyle>,
  /// Normal or bold weight.
  pub font_weight: Option<FontWeight>,
  /// In cases where text extends over more than one line, horizontal alignment and justify values can be different.
  /// The most typical case is for credits, such as:
  ///
  /// ```text
  /// Words and music by
  ///   Pat Songwriter
  /// ```
  /// Typically this type of credit is aligned to the right, so that the position information refers to the right-most part of the text.
  /// But in this example, the text is center-justified, not right-justified.
  ///
  /// The `halign` attribute is used in these situations. If it is not present, its value is the same as for the `justify` attribute.
  /// For elements where a justify attribute is not allowed, the default is implementation-dependent.
  pub halign: Option<LeftCenterRight>,
  /// Specifies an ID that is unique to the entire document.
  pub id: Option<Id>,
  /// Number of lines to use when striking through text.
  pub line_through: Option<NumberOfLines>,
  /// Number of lines to use when overlining text.
  pub overline: Option<NumberOfLines>,
  /// Indicates whether something is above or below another element, such as a note or a notation.
  pub placement: Option<AboveBelow>,
  /// Changes the horizontal position relative to the default position, either as computed by the individual program, or as overridden by the `default_x` attribute.
  /// Positive x is right and negative x is left. It should be interpreted in the context of the [Offset][super::Offset] element or directive attribute if those are present.
  pub relative_x: Option<Tenths>,
  /// Changes the vertical position relative to the default position, either as computed by the individual program, or as overridden by the `default_y` attribute.
  /// Positive y is up and negative y is down. It should be interpreted in the context of the `placement` attribute if that is present.
  pub relative_y: Option<Tenths>,
  /// Number of lines to use when underlining text.
  pub underline: Option<NumberOfLines>,
  /// Indicates vertical alignment to the top, middle, bottom, or baseline of the text. The default is implementation-dependent.
  pub valign: Option<Valign>,
}

/// The [DynamicsType] element specifies all possible dynamics available for use in a [Dynamics] element.
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
pub enum DynamicsType {
  /// The [P] element represents the piano dynamic.
  P(P),
  /// The [Pp] element represents the pianissimo dynamic.
  Pp(Pp),
  /// The [Ppp] element represents the pianississimo dynamic.
  Ppp(Ppp),
  /// The [Pppp] element represents the pianissississimo dynamic.
  Pppp(Pppp),
  /// The [Ppppp] element represents the pianississississimo dynamic.
  Ppppp(Ppppp),
  /// The [Pppppp] element represents the pianissississississimo dynamic.
  Pppppp(Pppppp),
  /// The [F] element represents the forte dynamic.
  F(F),
  /// The [Ff] element represents the fortissimo dynamic.
  Ff(Ff),
  /// The [Fff] element represents the fortississimo dynamic.
  Fff(Fff),
  /// The [Ffff] element represents the fortissississimo dynamic.
  Ffff(Ffff),
  /// The [Fffff] element represents the fortississississimo dynamic.
  Fffff(Fffff),
  /// The [Ffffff] element represents the fortissississississimo dynamic.
  Ffffff(Ffffff),
  /// The [Mp] element represents the mezzo-piano dynamic.
  Mp(Mp),
  /// The [Mf] element represents the mezzo-forte dynamic.
  Mf(Mf),
  /// The [Sf] element represents the sforzando dynamic.
  Sf(Sf),
  /// The [Sfp] element represents the sforzando-piano dynamic.
  Sfp(Sfp),
  /// The [Sfpp] element represents the sforzando-pianissimo dynamic.
  Sfpp(Sfpp),
  /// The [Fz] element represents the fortepiano dynamic.
  Fp(Fp),
  /// The [Rf] element represents the rinforzando dynamic.
  Rf(Rf),
  /// The [Rfz] element represents the rinforzando dynamic.
  Rfz(Rfz),
  /// The [Sfz] element represents the sforzato dynamic.
  Sfz(Sfz),
  /// The [Sffz] element represents the sforzatissimo dynamic.
  Sffz(Sffz),
  /// The [Fz] element represents the forzando dynamic.
  Fz(Fz),
  /// The [N] element represents the niente dynamic.
  N(N),
  /// The [Pf] element represents the pianoforte dynamic.
  Pf(Pf),
  /// The [Sfzp] element represents the sforzando-pianissimo dynamic.
  Sfzp(Sfzp),
  /// The [OtherDynamics] element allows other dynamic marks that are not covered here.
  #[rename("other-dynamics")]
  OtherDynamics(OtherDynamics),
}

/// Dynamics can be associated either with a note or a general musical direction.
///
/// ![Dynamics](https://hedgetechllc.github.io/musicxml/musicxml/elements/dynamics.png)
///
/// To avoid inconsistencies between and amongst the letter abbreviations for dynamics (what is sf vs. sfz, standing alone or with a trailing dynamic
/// that is not always piano), we use the actual letters as the names of these dynamic elements. The [OtherDynamics][super::OtherDynamics] element
/// allows other dynamic marks that are not covered here. Dynamics elements may also be combined to create marks not covered by a single element,
/// such as `<sf/><mp/>`.
///
/// These letter dynamic symbols are separated from crescendo, decrescendo, and wedge indications. Dynamic representation is inconsistent in scores.
/// Many things are assumed by the composer and left out, such as returns to original dynamics. The MusicXML format captures what is in the score, but does not
/// try to be optimal for analysis or synthesis of dynamics.
///
/// The `placement` attribute is used when the dynamics are associated with a [Note][super::Note]. It is ignored when the dynamics are associated with
/// a [Direction][super::Direction]. In that case the [Direction][super::Direction] element's `placement` attribute is used instead.
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
pub struct Dynamics {
  /// Element-specific attributes
  pub attributes: DynamicsAttributes,
  /// Element-specific content
  pub content: Vec<DynamicsType>,
}
