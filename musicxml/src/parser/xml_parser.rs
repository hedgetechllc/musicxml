use musicxml_internal::XmlElement;

enum TagType {
  Opening(XmlElement),
  Closing(XmlElement),
  SelfClosing(XmlElement),
  Ignored,
  Done,
}

fn read_tag_str(str: &mut std::str::Chars) -> TagType {
  let mut tag = XmlElement {
    name: String::new(),
    attributes: Vec::new(),
    elements: Vec::new(),
    text: String::new(),
  };
  let (mut is_closing, mut is_self_closing, mut in_attribute, mut in_string, mut in_tag, mut ignore) =
    (false, false, true, false, true, false);
  let (mut attribute, mut value) = (String::new(), String::new());
  for (i, c) in str.enumerate() {
    match c {
      '>' => {
        if !in_attribute {
          tag.attributes.push((attribute.clone(), value.clone()));
        }
        return if ignore {
          TagType::Ignored
        } else if is_closing {
          TagType::Closing(tag)
        } else if is_self_closing {
          TagType::SelfClosing(tag)
        } else {
          TagType::Opening(tag)
        };
      }
      '\r' => (),
      '\n' => (),
      '/' => {
        if in_string {
          value.push(c)
        } else if i == 0 {
          is_closing = true
        } else {
          is_self_closing = true
        }
      }
      '?' | '!' => {
        if i == 0 {
          ignore = true
        }
      }
      ' ' => {
        if in_tag {
          in_tag = false
        } else if in_string {
          value.push(c)
        } else {
          tag.attributes.push((attribute.clone(), value.clone()));
          in_attribute = true;
          attribute.clear();
          value.clear();
        }
      }
      '"' => in_string = !in_string,
      '=' => in_attribute = false,
      _ => {
        if in_tag {
          tag.name.push(c)
        } else if in_attribute {
          attribute.push(c)
        } else {
          value.push(c)
        }
      }
    }
  }
  TagType::Done
}

pub fn parse_to_string(xml: &XmlElement, depth: i16) -> String {
  let mut xml_str = String::new();
  if depth > 0 {
    xml_str += "\n";
  }
  for _ in 0..depth {
    xml_str += "  ";
  }
  xml_str += ["<", &xml.name].concat().as_str();
  for (key, value) in &xml.attributes {
    xml_str += [" ", &key, "=\"", &value, "\""].concat().as_str();
  }
  if xml.elements.is_empty() && xml.text.is_empty() {
    xml_str += "/>";
  } else {
    xml_str += ">";
    for element in &xml.elements {
      xml_str += parse_to_string(element, if depth >= 0 { depth + 1 } else { depth }).as_str();
    }
    if xml.text.is_empty() && depth >= 0 {
      xml_str += "\n";
      for _ in 0..depth {
        xml_str += "  ";
      }
    } else {
      xml_str += xml.text.as_str();
    }
    xml_str += ["</", &xml.name, ">"].concat().as_str();
  }
  xml_str
}

pub fn parse_from_string(str: &str) -> Result<XmlElement, String> {
  let mut it = str.chars();
  let mut open_tags: Vec<XmlElement> = Vec::new();
  while let Some(ch) = it.next() {
    if ch == '<' {
      match read_tag_str(&mut it) {
        TagType::Ignored => (),
        TagType::Opening(tag) => open_tags.push(tag),
        TagType::SelfClosing(tag) => match open_tags.last_mut() {
          Some(last_open_tag) => last_open_tag.elements.push(tag),
          None => return Err(format!("Root tag cannot be self-closing")),
        },
        TagType::Closing(tag) => {
          let mut element = open_tags.pop().unwrap();
          element.text.truncate(element.text.trim().len());
          if tag.name != element.name {
            return Err(format!(
              "Mismatched closing tag...expected '{}' but found '{}'",
              element.name, tag.name
            ));
          }
          if let Some(last_open_tag) = open_tags.last_mut() {
            last_open_tag.elements.push(element);
          } else {
            return Ok(element);
          }
        }
        TagType::Done => break,
      }
    } else if ch != '\r' && ch != '\n' && ch != '\t' {
      if let Some(item) = open_tags.last_mut() {
        if !item.text.is_empty() || ch != ' ' {
          item.text.push(ch)
        }
      }
    }
  }
  Err(format!("Missing one or more matched tags"))
}

#[cfg(test)]
mod xml_parser_tests {
  use super::*;

  #[test]
  fn serialize_valid_str() {
    let test_xml_str = "<outer2 attr=\"Test Attr\">
  <inner>Inner Test1</inner>
  <inner test=\"More Attr\">Inner Test2</inner>
  <inner2>
    <val1>123</val1>
    <val2>567</val2>
  </inner2>
  <self-close/>
</outer2>";
    let test_xml = XmlElement {
      name: String::from("outer2"),
      attributes: vec![(String::from("attr"), String::from("Test Attr"))],
      elements: vec![
        XmlElement {
          name: String::from("inner"),
          attributes: vec![],
          elements: vec![],
          text: String::from("Inner Test1"),
        },
        XmlElement {
          name: String::from("inner"),
          attributes: vec![(String::from("test"), String::from("More Attr"))],
          elements: vec![],
          text: String::from("Inner Test2"),
        },
        XmlElement {
          name: String::from("inner2"),
          attributes: vec![],
          elements: vec![
            XmlElement {
              name: String::from("val1"),
              attributes: vec![],
              elements: vec![],
              text: String::from("123"),
            },
            XmlElement {
              name: String::from("val2"),
              attributes: vec![],
              elements: vec![],
              text: String::from("567"),
            },
          ],
          text: String::new(),
        },
        XmlElement {
          name: String::from("self-close"),
          attributes: vec![],
          elements: vec![],
          text: String::new(),
        },
      ],
      text: String::new(),
    };
    let result = parse_to_string(&test_xml, 0);
    assert_eq!(result.as_str(), test_xml_str);
  }

  #[test]
  fn deserialize_valid_str() {
    let test_xml = "
    <outer2 attr=\"Test Attr\">
      <inner>Inner Test1</inner>
      <inner test=\"More Attr\">Inner Test2</inner>
      <inner2>
        <val1>123</val1>
        <val2>567</val2>
      </inner2>
    </outer2>";
    let result = parse_from_string(test_xml);
    assert!(result.is_ok());
    assert_eq!(
      result.unwrap(),
      XmlElement {
        name: String::from("outer2"),
        attributes: vec![(String::from("attr"), String::from("Test Attr"))],
        elements: vec![
          XmlElement {
            name: String::from("inner"),
            attributes: vec![],
            elements: vec![],
            text: String::from("Inner Test1")
          },
          XmlElement {
            name: String::from("inner"),
            attributes: vec![(String::from("test"), String::from("More Attr"))],
            elements: vec![],
            text: String::from("Inner Test2")
          },
          XmlElement {
            name: String::from("inner2"),
            attributes: vec![],
            elements: vec![
              XmlElement {
                name: String::from("val1"),
                attributes: vec![],
                elements: vec![],
                text: String::from("123")
              },
              XmlElement {
                name: String::from("val2"),
                attributes: vec![],
                elements: vec![],
                text: String::from("567")
              },
            ],
            text: String::new()
          },
        ],
        text: String::new()
      }
    );
  }

  #[test]
  fn serialize_valid_unicode_str() {
    let test_xml_str = "<element><test1>Waltz in E♭ Major</test1><test2>Frédéric François Chopin</test2></element>";
    let test_xml = XmlElement {
      name: String::from("element"),
      attributes: vec![],
      elements: vec![
        XmlElement {
          name: String::from("test1"),
          attributes: vec![],
          elements: vec![],
          text: String::from("Waltz in E♭ Major"),
        },
        XmlElement {
          name: String::from("test2"),
          attributes: vec![],
          elements: vec![],
          text: String::from("Frédéric François Chopin"),
        },
      ],
      text: String::new(),
    };
    let result = parse_to_string(&test_xml, -1);
    assert_eq!(result.as_str(), test_xml_str);
  }

  #[test]
  fn deserialize_valid_unicode_str() {
    let test_xml = "<element><test1>Waltz in E♭ Major</test1><test2>Frédéric François Chopin</test2></element>";
    let result = parse_from_string(test_xml);
    assert!(result.is_ok());
    assert_eq!(
      result.unwrap(),
      XmlElement {
        name: String::from("element"),
        attributes: vec![],
        elements: vec![
          XmlElement {
            name: String::from("test1"),
            attributes: vec![],
            elements: vec![],
            text: String::from("Waltz in E♭ Major")
          },
          XmlElement {
            name: String::from("test2"),
            attributes: vec![],
            elements: vec![],
            text: String::from("Frédéric François Chopin")
          },
        ],
        text: String::new()
      }
    );
  }
}
