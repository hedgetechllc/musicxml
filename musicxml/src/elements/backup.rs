use super::{Duration, Footnote, Level};
use alloc::{string::String, vec::Vec};
use musicxml_internal::*;
use musicxml_macros::*;

/// Contents of the [Backup] element.
#[derive(Debug, PartialEq, Eq, ContentDeserialize, ContentSerialize)]
pub struct BackupContents {
  /// The [Duration] element specifies the number of divisions to move back.
  pub duration: Duration,
  /// The [Footnote] element specifies editorial information or lyrics content.
  pub footnote: Option<Footnote>,
  /// The [Level] element specifies the editorial level of a score or part.
  pub level: Option<Level>,
}

/// The [Backup] and [Forward][super::Forward] elements are required to coordinate multiple voices in one part, including music on multiple staves.
///
/// ![Backup](https://hedgetechllc.github.io/musicxml/musicxml/elements/backup.png)
///
/// The [Backup] element is generally used to move between voices and staves. Thus it does not include [Voice][super::Voice] or [Staff][super::Staff] elements.
/// Duration values should always be positive, and should not cross measure boundaries or mid-measure changes in the [Divisions][super::Divisions] value.
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
pub struct Backup {
  /// Element-specific attributes
  pub attributes: (),
  #[flatten]
  /// Element-specific content
  pub content: BackupContents,
}

#[cfg(test)]
mod backup_tests {
  use super::*;
  use crate::parser::parse_from_xml_str;
  use crate::{
    datatypes::PositiveDivisions,
    elements::{FootnoteAttributes, LevelAttributes},
  };

  #[test]
  fn deserialize_valid1() {
    let result = parse_from_xml_str::<Backup>(
      "
    <backup>
      <duration>2</duration>
      <footnote>Footnote</footnote>
      <level>Level</level>
    </backup>",
    );
    assert!(result.is_ok());
    assert_eq!(
      result.unwrap(),
      Backup {
        attributes: (),
        content: BackupContents {
          duration: Duration {
            attributes: (),
            content: PositiveDivisions(2)
          },
          footnote: Some(Footnote {
            attributes: FootnoteAttributes::default(),
            content: String::from("Footnote")
          }),
          level: Some(Level {
            attributes: LevelAttributes::default(),
            content: String::from("Level")
          }),
        }
      }
    );
  }
}
