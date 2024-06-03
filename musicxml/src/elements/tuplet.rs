use super::{TupletActual, TupletNormal};
use crate::datatypes::{AboveBelow, Id, LineShape, NumberLevel, ShowTuplet, StartStop, Tenths, YesNo};
use musicxml_internal::*;
use musicxml_macros::*;

/// Attributes pertaining to the [Tuplet] element.
#[derive(Debug, PartialEq, Eq, AttributeDeserialize, AttributeSerialize)]
pub struct TupletAttributes {
  /// Indicates if this is the start or stop of the tuplet.
  pub r#type: StartStop,
  /// Specifies whether or not brackets are put around a symbol for an editorial indication. If not specified, it is left to application defaults.
  pub bracket: Option<YesNo>,
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
  /// Specifies an ID that is unique to the entire document.
  pub id: Option<Id>,
  /// Used to specify whether the bracket is straight or in the older curved or slurred style. It is straight if not specified.
  pub line_shape: Option<LineShape>,
  /// Distinguishes nested tuplets.
  pub number: Option<NumberLevel>,
  /// Indicates whether something is above or below another element, such as a note or a notation.
  pub placement: Option<AboveBelow>,
  /// Changes the horizontal position relative to the default position, either as computed by the individual program, or as overridden by the `default_x` attribute.
  /// Positive x is right and negative x is left. It should be interpreted in the context of the [Offset][super::Offset] element or directive attribute if those are present.
  pub relative_x: Option<Tenths>,
  /// Changes the vertical position relative to the default position, either as computed by the individual program, or as overridden by the `default_y` attribute.
  /// Positive y is up and negative y is down. It should be interpreted in the context of the `placement` attribute if that is present.
  pub relative_y: Option<Tenths>,
  /// Used to display either the number of actual notes, the number of both actual and normal notes, or neither. It is actual if not specified.
  pub show_number: Option<ShowTuplet>,
  /// Used to display either the actual type, both the actual and normal types, or neither. It is none if not specified.
  pub show_type: Option<ShowTuplet>,
}

/// Contents of the [Tuplet] element.
#[derive(Debug, Default, PartialEq, Eq, ContentDeserialize, ContentSerialize)]
pub struct TupletContents {
  /// The [TupletActual] element specifies the number of notes in the tuplet.
  pub tuplet_actual: Option<TupletActual>,
  /// The [TupletNormal] element specifies the number of notes in the tuplet as represented by the normal, i.e. not tuplet, note type.
  pub tuplet_normal: Option<TupletNormal>,
}

/// A [Tuplet] element is present when a tuplet is to be displayed graphically, in addition to the sound data provided
/// by the [TimeModification][super::TimeModification] elements.
///
/// ![Tuplet](tuplet.png)
///
/// Whereas a [TimeModification][super::TimeModification] element shows how the cumulative, sounding effect of tuplets and double-note tremolos compare
/// to the written note type, the [Tuplet] element describes how this is displayed. The [Tuplet] element also provides more detailed representation information
/// than the [TimeModification][super::TimeModification] element, and is needed to represent nested tuplets and other complex tuplets accurately.
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
pub struct Tuplet {
  /// Element-specific attributes
  pub attributes: TupletAttributes,
  #[flatten]
  /// Element-specific content
  pub content: TupletContents,
}
