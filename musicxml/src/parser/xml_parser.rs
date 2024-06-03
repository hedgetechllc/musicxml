use musicxml_internal::XmlElement;

enum TagType {
  Opening { tag_length: usize, tag: XmlElement },
  Closing { tag_length: usize, tag: XmlElement },
  SelfClosing { tag_length: usize, tag: XmlElement },
  Ignored { tag_length: usize },
  Done,
}

fn read_tag_str(str: &str) -> TagType {
  let mut tag = XmlElement {
    name: String::new(),
    attributes: Vec::new(),
    elements: Vec::new(),
    text: String::new(),
  };
  let (mut is_closing, mut is_self_closing, mut in_attribute, mut in_string, mut in_tag, mut ignore) =
    (false, false, true, false, true, false);
  let (mut attribute, mut value) = (String::new(), String::new());
  for (i, c) in str.chars().enumerate() {
    match c {
      '>' => {
        if !in_attribute {
          tag.attributes.push((attribute.clone(), value.clone()));
        }
        return if ignore {
          TagType::Ignored { tag_length: i + 1 }
        } else if is_closing {
          TagType::Closing { tag_length: i + 1, tag }
        } else if is_self_closing {
          TagType::SelfClosing { tag_length: i + 1, tag }
        } else {
          TagType::Opening { tag_length: i + 1, tag }
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

pub fn write_xml_to_str(xml: &XmlElement, depth: i16) -> String {
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
      xml_str += write_xml_to_str(element, if depth >= 0 { depth + 1 } else { depth }).as_str();
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

pub fn write_xml_to_file(path: &str, xml: &XmlElement, pretty_print: bool) -> Result<(), String> {
  std::fs::write(path, write_xml_to_str(xml, if pretty_print { 0 } else { -1 })).map_err(|e| e.to_string())
}

pub fn parse_xml_from_str(mut str: &str) -> Result<XmlElement, String> {
  let mut open_tags: Vec<XmlElement> = Vec::new();
  while !str.is_empty() {
    if str.starts_with('<') {
      match read_tag_str(&str[1..]) {
        TagType::Ignored { tag_length } => str = &str[tag_length..],
        TagType::Opening { tag_length, tag } => {
          str = &str[tag_length..];
          open_tags.push(tag)
        }
        TagType::SelfClosing { tag_length, tag } => {
          str = &str[tag_length..];
          match open_tags.last_mut() {
            Some(last_open_tag) => last_open_tag.elements.push(tag),
            None => return Err(format!("Root tag cannot be self-closing")),
          }
        }
        TagType::Closing { tag_length, tag } => {
          str = &str[tag_length..];
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
        TagType::Done => str = &str[str.chars().count() - 1..],
      }
    } else if !str.starts_with('\r') && !str.starts_with('\n') && !str.starts_with('\t') {
      if let Some(item) = open_tags.last_mut() {
        let ch = str.chars().next().unwrap();
        if !item.text.is_empty() || ch != ' ' {
          item.text.push(ch)
        }
      }
    }
    str = &str[1..];
  }
  Err(format!("Missing one or more matched tags"))
}

pub fn parse_xml_from_file(path: &str) -> Result<XmlElement, String> {
  if let Ok(contents) = std::fs::read_to_string(path) {
    parse_xml_from_str(&contents)
  } else {
    Err(format!("Unable to open file at '{}'", path))
  }
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
    let result = write_xml_to_str(&test_xml, 0);
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
    let result = parse_xml_from_str(test_xml);
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
  fn deserialize_valid_file() {
    let result = parse_xml_from_file("tests/Grande Valse Brillante.musicxml");
    assert!(result.is_ok());
  }
}
