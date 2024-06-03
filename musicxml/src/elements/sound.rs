use super::{InstrumentChange, MidiDevice, MidiInstrument, Offset, Play, Swing};
use crate::datatypes::{Divisions, Id, NonNegativeDecimal, RotationDegrees, TimeOnly, Token, YesNo, YesNoNumber};
use musicxml_internal::*;
use musicxml_macros::*;

/// Attributes pertaining to the [Sound] element.
#[derive(Debug, Default, PartialEq, Eq, AttributeDeserialize, AttributeSerialize)]
pub struct SoundAttributes {
  /// Indicates the end point for a forward jump to a coda sign. If there are multiple jumps, the value of these parameters can be used to name and distinguish them.
  pub coda: Option<Token>,
  /// Indicates to go back to the beginning of the movement. When used it always has the value "yes".
  ///
  /// By default, a `dacapo` attribute indicates that the jump should occur the first time through. The times that jumps occur can be changed by using the `time_only` attribute.
  pub dacapo: Option<YesNo>,
  /// Indicates the starting point for a backward jump to a segno sign. If there are multiple jumps, the value of these parameters can be used to name and distinguish them.
  ///
  /// By default, a `dalsegno` attribute indicates that the jump should occur the first time through. The times that jumps occur can be changed by using the `time_only` attribute.
  pub dalsegno: Option<Token>,
  /// Effects playback of the the common right piano pedal and its MIDI controller equivalent. The "yes" value indicates the pedal is depressed; "no" indicates the pedal is released.
  /// A numeric value from 0 to 100 may also be used for half-pedaling. This value is the percentage that the pedal is depressed. A value of 0 is equivalent to "no", and a value of 100 is equivalent to "yes".
  pub damper_pedal: Option<YesNoNumber>,
  /// If the `segno` or `coda` attributes are used, the `divisions` attribute can be used to indicate the number of divisions per quarter note. Otherwise sound and MIDI generating programs may have to recompute this.
  pub divisions: Option<Divisions>,
  /// Dynamics (or MIDI velocity) are expressed as a percentage of the default forte value (90 for MIDI 1.0).
  pub dynamics: Option<NonNegativeDecimal>,
  /// Allows placing of sound in a 3-D space relative to the listener, expressed in degrees ranging from -180 to 180. 0 is level with the listener, 90 is directly above, and -90 is directly below.
  ///
  /// Deprecated as of Version 2.0. The [Elevation][super::Elevation] element in the [MidiInstrument] element should be used instead. If both are present, the [Elevation][super::Elevation] element takes priority.
  pub elevation: Option<RotationDegrees>,
  /// Follows the final note or rest in a movement with a da capo or dal segno direction. If numeric, the value represents the actual duration of the final note or rest, which can be ambiguous in written notation
  /// and different among parts and voices. The value may also be "yes" to indicate no change to the final duration.
  pub fine: Option<Token>,
  /// Indicates that a forward repeat sign is implied but not displayed. It is used for example in two-part forms with repeats, such as a minuet and trio where no repeat is displayed at the start of the trio.
  /// This usually occurs after a barline. When used it always has the value of "yes".
  pub forward_repeat: Option<YesNo>,
  /// Specifies an ID that is unique to the entire document.
  pub id: Option<Id>,
  /// Allows placing of sound in a 3-D space relative to the listener, expressed in degrees ranging from -180 to 180. 0 is straight ahead, -90 is hard left, 90 is hard right, and -180 and 180 are directly behind the listener.
  ///
  /// Deprecated as of Version 2.0. The [Pan][super::Pan] element in the [MidiInstrument] element should be used instead. If both are present, the [Pan][super::Pan] element takes priority.
  pub pan: Option<RotationDegrees>,
  /// Affects all following notes. "Yes" indicates pizzicato, "no" indicates arco.
  pub pizzicato: Option<YesNo>,
  /// Indicates the end point for a backward jump to a segno sign. If there are multiple jumps, the value of these parameters can be used to name and distinguish them.
  pub segno: Option<Token>,
  /// Effects playback of the the common left piano pedal and its MIDI controller equivalent. The "yes" value indicates the pedal is depressed; "no" indicates the pedal is released.
  /// A numeric value from 0 to 100 may also be used for half-pedaling. This value is the percentage that the pedal is depressed. A value of 0 is equivalent to "no", and a value of 100 is equivalent to "yes".
  pub soft_pedal: Option<YesNoNumber>,
  /// Effects playback of the the common center piano pedal and its MIDI controller equivalent. The "yes" value indicates the pedal is depressed; "no" indicates the pedal is released.
  /// A numeric value from 0 to 100 may also be used for half-pedaling. This value is the percentage that the pedal is depressed. A value of 0 is equivalent to "no", and a value of 100 is equivalent to "yes".
  pub sostenuto_pedal: Option<YesNoNumber>,
  /// Tempo is expressed in quarter notes per minute. If 0, the sound-generating program should prompt the user at the time of compiling a sound (MIDI) file.
  pub tempo: Option<NonNegativeDecimal>,
  /// Indicates which times to apply the sound element if the [Sound] element applies only particular times through a repeat.
  pub time_only: Option<TimeOnly>,
  /// Indicates the starting point for a forward jump to a coda sign. If there are multiple jumps, the value of these parameters can be used to name and distinguish them.
  ///
  /// By default, a `tocoda` attribute indicates the jump should occur the second time through. The times that jumps occur can be changed by using the `time_only` attribute.
  pub tocoda: Option<Token>,
}

#[derive(Debug, Default, PartialEq, Eq, ContentDeserialize, ContentSerialize)]
pub struct SoundContents {
  pub instrument_change: Vec<InstrumentChange>,
  pub midi_device: Vec<MidiDevice>,
  pub midi_instrument: Vec<MidiInstrument>,
  pub play: Vec<Play>,
  pub swing: Option<Swing>,
  pub offset: Option<Offset>,
}

/// The [Sound] element contains general playback parameters.
///
/// They can stand alone within a part/measure, or be a component element within a direction.
///
/// Instrument changes, MIDI devices, MIDI instruments, and playback techniques are changed using the [InstrumentChange],
/// [MidiDevice], [MidiInstrument], and [Play] elements. When there are multiple instances of these elements, they should be
/// grouped together by instrument using the `id` attribute values.
///
/// The [Offset] element is used to indicate that the sound takes place offset from the current score position.
/// If the [Sound] element is a child of a [Direction][super::Direction] element, the sound [Offset] element overrides the
/// direction [Offset] element if both elements are present. Note that the offset reflects the intended musical position for
/// the change in sound. It should not be used to compensate for latency issues in particular hardware configurations.
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
pub struct Sound {
  /// Element-specific attributes
  pub attributes: SoundAttributes,
  #[flatten]
  /// Element-specific content
  pub content: SoundContents,
}
