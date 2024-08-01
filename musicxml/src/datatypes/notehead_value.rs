use alloc::string::String;
use musicxml_internal::{DatatypeDeserializer, DatatypeSerializer};
use musicxml_macros::{DatatypeDeserialize, DatatypeSerialize};

/// Indicates shapes other than the open and closed ovals associated with note durations.
///
/// The values do, re, mi, fa, fa up, so, la, and ti correspond to Aikin's 7-shape system.
/// The "fa up" shape is typically used with upstems; the "fa" shape is typically used with downstems or no stems.
///
/// The arrow shapes differ from triangle and inverted triangle by being centered on the stem.
/// Slashed and back-slashed notes include both the normal notehead and a slash.
/// The triangle shape has the tip of the triangle pointing up;
/// the inverted triangle shape has the tip of the triangle pointing down.
/// The left triangle shape is a right triangle with the hypotenuse facing up and to the left.
///
/// The "other" notehead covers noteheads other than those listed here.
/// It is usually used in combination with the `smufl` attribute to specify a particular SMuFL notehead.
/// The `smufl` attribute may be used with any notehead value to help specify the appearance of symbols
/// that share the same MusicXML semantics. Noteheads in the SMuFL Note name noteheads and Note name noteheads
/// supplement ranges (U+E150–U+E1AF and U+EEE0–U+EEFF) should not use the `smufl` attribute or the "other" value,
/// but instead use the [NoteheadText][crate::elements::NoteheadText] element.
#[derive(Debug, PartialEq, Eq, DatatypeDeserialize, DatatypeSerialize)]
pub enum NoteheadValue {
  /// ![ArrowDown](https://hedgetechllc.github.io/musicxml/musicxml/datatypes/notehead-value-arrow-down.png)
  #[rename("arrow down")]
  ArrowDown,
  /// ![ArrowUp](https://hedgetechllc.github.io/musicxml/musicxml/datatypes/notehead-value-arrow-up.png)
  #[rename("arrow up")]
  ArrowUp,
  /// ![BackSlashed](https://hedgetechllc.github.io/musicxml/musicxml/datatypes/notehead-value-back-slashed.png)
  #[rename("back slashed")]
  BackSlashed,
  /// ![CircleDot](https://hedgetechllc.github.io/musicxml/musicxml/datatypes/notehead-value-circle-dot.png)
  #[rename("circle dot")]
  CircleDot,
  /// ![CircleX](https://hedgetechllc.github.io/musicxml/musicxml/datatypes/notehead-value-circle-x.png)
  #[rename("circle-x")]
  CircleX,
  /// ![Circled](https://hedgetechllc.github.io/musicxml/musicxml/datatypes/notehead-value-circled.png)
  Circled,
  /// ![Cluster](https://hedgetechllc.github.io/musicxml/musicxml/datatypes/notehead-value-cluster.png)
  Cluster,
  /// ![Cross](https://hedgetechllc.github.io/musicxml/musicxml/datatypes/notehead-value-cross.png)
  Cross,
  /// ![Diamond](https://hedgetechllc.github.io/musicxml/musicxml/datatypes/notehead-value-diamond.png)
  Diamond,
  /// As in Aikin's 7-shape system:
  ///
  /// ![Do](https://hedgetechllc.github.io/musicxml/musicxml/datatypes/notehead-value-do.png)
  Do,
  /// As in Aikin's 7-shape system, typically used with downstems or no stems:
  ///
  /// ![Fa](https://hedgetechllc.github.io/musicxml/musicxml/datatypes/notehead-value-fa.png)
  Fa,
  /// As in Aikin's 7-shape system, typically used with upstems:
  ///
  /// ![FaUp](https://hedgetechllc.github.io/musicxml/musicxml/datatypes/notehead-value-fa-up.png)
  #[rename("fa up")]
  FaUp,
  /// ![InvertedTriangle](https://hedgetechllc.github.io/musicxml/musicxml/datatypes/notehead-value-inverted-triangle.png)
  #[rename("inverted triangle")]
  InvertedTriangle,
  /// As in Aikin's 7-shape system:
  ///
  /// ![La](https://hedgetechllc.github.io/musicxml/musicxml/datatypes/notehead-value-la.png)
  La,
  /// ![LeftTriangle](https://hedgetechllc.github.io/musicxml/musicxml/datatypes/notehead-value-left-triangle.png)
  #[rename("left triangle")]
  LeftTriangle,
  /// As in Aikin's 7-shape system:
  ///
  /// ![Mi](https://hedgetechllc.github.io/musicxml/musicxml/datatypes/notehead-value-mi.png)
  Mi,
  /// ![None](https://hedgetechllc.github.io/musicxml/musicxml/datatypes/notehead-value-none.png)
  None,
  /// ![Normal](https://hedgetechllc.github.io/musicxml/musicxml/datatypes/notehead-value-normal.png)
  Normal,
  /// As in Aikin's 7-shape system:
  ///
  /// ![Re](https://hedgetechllc.github.io/musicxml/musicxml/datatypes/notehead-value-re.png)
  Re,
  /// ![Rectangle](https://hedgetechllc.github.io/musicxml/musicxml/datatypes/notehead-value-rectangle.png)
  Rectangle,
  /// ![Slash](https://hedgetechllc.github.io/musicxml/musicxml/datatypes/notehead-value-slash.png)
  Slash,
  /// ![Slashed](https://hedgetechllc.github.io/musicxml/musicxml/datatypes/notehead-value-slashed.png)
  Slashed,
  /// As in Aikin's 7-shape system:
  ///
  /// ![So](https://hedgetechllc.github.io/musicxml/musicxml/datatypes/notehead-value-so.png)
  So,
  /// ![Square](https://hedgetechllc.github.io/musicxml/musicxml/datatypes/notehead-value-square.png)
  Square,
  /// As in Aikin's 7-shape system:
  ///
  /// ![Ti](https://hedgetechllc.github.io/musicxml/musicxml/datatypes/notehead-value-ti.png)
  Ti,
  /// ![Triangle](https://hedgetechllc.github.io/musicxml/musicxml/datatypes/notehead-value-triangle.png)
  Triangle,
  /// ![X](https://hedgetechllc.github.io/musicxml/musicxml/datatypes/notehead-value-x.png)
  X,
  /// Noteheads other than those listed here.
  ///
  /// This value is usually used in combination with the `smufl` attribute to specify a particular SMuFL notehead.
  /// The `smufl` attribute may also be used with any notehead value to help specify the appearance of symbols that
  /// share the same MusicXML semantics. Noteheads in the SMuFL Note name noteheads and Note name noteheads
  /// supplement ranges (U+E150–U+E1AF and U+EEE0–U+EEFF) should not use the `smufl` attribute or the other value,
  /// but instead use the [NoteheadText][crate::elements::NoteheadText] element.
  Other,
}
