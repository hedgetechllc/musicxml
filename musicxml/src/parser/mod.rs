use musicxml_internal::{ElementDeserializer, ElementSerializer};

mod xml_parser;

pub fn parse_to_xml_str<T: ElementSerializer>(data: &T, pretty_print: bool) -> String {
  let xml = T::serialize(data);
  xml_parser::write_xml_to_str(&xml, if pretty_print { 0 } else { -1 })
}

pub fn parse_to_xml_file<T: ElementSerializer>(path: &str, data: &T, pretty_print: bool) -> Result<(), String> {
  let xml = T::serialize(data);
  xml_parser::write_xml_to_file(path, &xml, pretty_print)
}

pub fn parse_from_xml_str<T: ElementDeserializer>(str: &str) -> Result<T, String> {
  let xml = xml_parser::parse_xml_from_str(str)?;
  T::deserialize(&xml)
}

pub fn parse_from_xml_file<T: ElementDeserializer>(path: &str) -> Result<T, String> {
  let xml = xml_parser::parse_xml_from_file(path)?;
  T::deserialize(&xml)
}
