use musicxml_internal::*;
use musicxml_macros::*;

/// The [AccordionMiddle] element indicates the presence of 1 to 3 dots in the middle (8') section of the registration symbol.
/// 
/// ![AccordionMiddle](accordion-middle.png)
/// 
/// This element is omitted if no dots are present.
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
#[rename("accordion-middle")]
pub struct AccordionMiddle {
  /// Element-specific attributes
  pub attributes: (),
  /// Element-specific content
  pub content: crate::datatypes::AccordionMiddle,
}

#[cfg(test)]
mod accordion_middle_tests {
  use super::*;
  use crate::parser::parse_from_xml_str;

  #[test]
  fn deserialize_valid1() {
    let result = parse_from_xml_str::<AccordionMiddle>("<accordion-middle>2</accordion-middle>");
    assert!(result.is_ok());
    assert_eq!(
      result.unwrap(),
      AccordionMiddle {
        attributes: (),
        content: crate::datatypes::AccordionMiddle(2),
      }
    );
  }
}
