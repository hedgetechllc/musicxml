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
  /// ![ArrowDown](notehead-value-arrow-down.png)
  #[rename("arrow down")]
  ArrowDown,
  /// ![ArrowUp](notehead-value-arrow-up.png)
  #[rename("arrow up")]
  ArrowUp,
  /// ![BackSlashed](notehead-value-back-slashed.png)
  #[rename("back slashed")]
  BackSlashed,
  /// ![CircleDot](notehead-value-circle-dot.png)
  #[rename("circle dot")]
  CircleDot,
  /// ![CircleX](notehead-value-circle-x.png)
  #[rename("circle-x")]
  CircleX,
  /// ![Circled](notehead-value-circled.png)
  Circled,
  /// ![Cluster](notehead-value-cluster.png)
  Cluster,
  /// ![Cross](notehead-value-cross.png)
  Cross,
  /// ![Diamond](notehead-value-diamond.png)
  Diamond,
  /// As in Aikin's 7-shape system:
  /// 
  /// ![Do](notehead-value-do.png)
  Do,
  /// As in Aikin's 7-shape system, typically used with downstems or no stems:
  /// 
  /// ![Fa](notehead-value-fa.png)
  Fa,
  /// As in Aikin's 7-shape system, typically used with upstems:
  /// 
  /// ![FaUp](notehead-value-fa-up.png)
  #[rename("fa up")]
  FaUp,
  /// ![InvertedTriangle](notehead-value-inverted-triangle.png)
  #[rename("inverted triangle")]
  InvertedTriangle,
  /// As in Aikin's 7-shape system:
  /// 
  /// ![La](notehead-value-la.png)
  La,
  /// ![LeftTriangle](notehead-value-left-triangle.png)
  #[rename("left triangle")]
  LeftTriangle,
  /// As in Aikin's 7-shape system:
  /// 
  /// ![Mi](notehead-value-mi.png)
  Mi,
  /// ![None](notehead-value-none.png)
  None,
  /// ![Normal](notehead-value-normal.png)
  Normal,
  /// As in Aikin's 7-shape system:
  /// 
  /// ![Re](notehead-value-re.png)
  Re,
  /// ![Rectangle](notehead-value-rectangle.png)
  Rectangle,
  /// ![Slash](notehead-value-slash.png)
  Slash,
  /// ![Slashed](notehead-value-slashed.png)
  Slashed,
  /// As in Aikin's 7-shape system:
  /// 
  /// ![So](notehead-value-so.png)
  So,
  /// ![Square](notehead-value-square.png)
  Square,
  /// As in Aikin's 7-shape system:
  /// 
  /// ![Ti](notehead-value-ti.png)
  Ti,
  /// ![Triangle](notehead-value-triangle.png)
  Triangle,
  /// ![X](notehead-value-x.png)
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
