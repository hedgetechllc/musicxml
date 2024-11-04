use crate::elements::{ScorePartwise, ScoreTimewise};
use alloc::{collections::BTreeMap, string::String, vec::Vec};
use musicxml_internal::{ElementDeserializer, ElementSerializer, XmlElement};

#[cfg(feature = "std")]
extern crate std;

#[cfg(feature = "std")]
use {alloc::string::ToString, std::io::Write};

mod xml_parser;
mod zip_parser;

#[inline]
fn is_mxl_data(data: Option<&[u8]>) -> bool {
  if let Some(data) = data {
    *data == [0x50, 0x4b, 0x03, 0x04]
  } else {
    false
  }
}

fn get_musicxml_contents(data: Vec<u8>) -> Result<String, String> {
  let mut contents = String::new();
  if is_mxl_data(data.get(0..4)) {
    let mut xml_path: Option<String> = None;
    let mut mxl_data = zip_parser::ZipData::new();
    mxl_data.content = data;
    let archive = zip_parser::ZipArchive::new(&mut mxl_data);
    for file_name in archive.iter() {
      if file_name == "META-INF/container.xml" {
        let container = xml_parser::parse_from_string(archive.read_file_to_string(file_name)?.as_str())?;
        xml_path = container
          .elements
          .iter()
          .find(|&el| el.name == "rootfiles")
          .and_then(|el| el.elements.iter().find(|&el| el.name == "rootfile"))
          .and_then(|el| el.attributes.iter().find(|&attr| attr.0 == "full-path"))
          .map(|attr| attr.1.clone());
      }
    }
    if let Some(full_path) = &xml_path {
      contents = archive.read_file_to_string(full_path)?;
    } else {
      Err(String::from("Cannot find MusicXML file in compressed archive"))?;
    }
  } else {
    contents = String::from_utf8(data).or_else(|_| Err(String::from("Invalid UTF-8 data in MusicXML content")))?;
  }
  Ok(contents)
}

#[cfg(feature = "std")]
fn get_musicxml_contents_from_file(path: &str) -> Result<String, String> {
  get_musicxml_contents(std::fs::read(path).map_err(|e| e.to_string())?)
}

#[cfg(not(feature = "std"))]
fn get_musicxml_contents_from_file(_path: &str) -> Result<String, String> {
  Err(String::from(
    "Reading MusicXML files is not supported in a 'no_std' environment",
  ))
}

fn put_musicxml_contents(xml: &XmlElement, pretty_print: bool) -> Vec<u8> {
  let mut buffer = Vec::new();
  buffer.extend_from_slice(b"<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n");
  if xml.name == "score-partwise" {
    buffer.extend_from_slice(b"<!DOCTYPE score-partwise PUBLIC \"-//Recordare//DTD MusicXML 3.0 Partwise//EN\" \"http://www.musicxml.org/dtds/partwise.dtd\">\n");
  } else if xml.name == "score-timewise" {
    buffer.extend_from_slice(b"<!DOCTYPE score-timewise PUBLIC \"-//Recordare//DTD MusicXML 3.0 Timewise//EN\" \"http://www.musicxml.org/dtds/timewise.dtd\">\n");
  }
  buffer.extend_from_slice(xml_parser::parse_to_string(xml, if pretty_print { 0 } else { -1 }).as_ref());
  buffer
}

fn write_musicxml_contents(xml: &XmlElement, compressed: bool, pretty_print: bool) -> Vec<u8> {
  if compressed {
    let mut archiver = zip_parser::ZipArchiver::new();
    archiver.start_file("META-INF/container.xml");
    archiver.write_data(b"<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<container>\n  <rootfiles>\n    <rootfile full-path=\"score.musicxml\" media-type=\"application/vnd.recordare.musicxml+xml\"/>\n  </rootfiles>\n</container>");
    archiver.start_file("score.musicxml");
    archiver.write_data(put_musicxml_contents(xml, pretty_print).as_slice());
    archiver.finish()
  } else {
    put_musicxml_contents(xml, pretty_print)
  }
}

#[cfg(feature = "std")]
fn write_musicxml_contents_to_file(
  path: &str,
  xml: &XmlElement,
  compressed: bool,
  pretty_print: bool,
) -> Result<(), String> {
  let mut file = std::fs::OpenOptions::new()
    .write(true)
    .create(true)
    .truncate(true)
    .open(path)
    .map_err(|e| e.to_string())?;
  file
    .write_all(&write_musicxml_contents(xml, compressed, pretty_print))
    .map_err(|e| e.to_string())
}

#[cfg(not(feature = "std"))]
fn write_musicxml_contents_to_file(
  _path: &str,
  _xml: &XmlElement,
  _compressed: bool,
  _pretty_print: bool,
) -> Result<(), String> {
  Err(String::from(
    "Writing MusicXML files is not supported in a 'no_std' environment",
  ))
}

fn convert_xml_partwise_to_timewise(xml: XmlElement) -> Result<XmlElement, String> {
  if xml.name == "score-timewise" {
    Ok(xml)
  } else if xml.name == "score-partwise" {
    let mut measures: BTreeMap<usize, XmlElement> = BTreeMap::new();
    let mut converted_xml = XmlElement {
      name: String::from("score-timewise"),
      attributes: xml.attributes,
      elements: Vec::new(),
      text: xml.text,
    };
    for element in xml.elements {
      if element.name == "part" {
        for measure_element in element.elements {
          let measure_number: usize = measure_element
            .attributes
            .iter()
            .find(|(key, _)| key == "number")
            .ok_or("Missing \"number\" attribute in <measure> element")?
            .1
            .parse()
            .map_err(|err| format!("Invalid \"number\" attribute in <measure> element: {err}"))?;
          measures
            .entry(measure_number)
            .or_insert_with(|| XmlElement {
              name: measure_element.name,
              attributes: measure_element.attributes,
              elements: Vec::new(),
              text: measure_element.text,
            })
            .elements
            .push(XmlElement {
              name: element.name.clone(),
              attributes: element.attributes.clone(),
              elements: measure_element.elements,
              text: element.text.clone(),
            });
        }
      } else {
        converted_xml.elements.push(element);
      }
    }
    measures
      .into_values()
      .for_each(|measure| converted_xml.elements.push(measure));
    Ok(converted_xml)
  } else {
    Err(String::from(
      "Root element in a MusicXML file must be either <score-partwise> or <score-timewise>",
    ))
  }
}

fn convert_xml_timewise_to_partwise(xml: XmlElement) -> Result<XmlElement, String> {
  if xml.name == "score-partwise" {
    Ok(xml)
  } else if xml.name == "score-timewise" {
    let mut parts: BTreeMap<String, XmlElement> = BTreeMap::new();
    let mut converted_xml = XmlElement {
      name: String::from("score-partwise"),
      attributes: xml.attributes,
      elements: Vec::new(),
      text: xml.text,
    };
    for element in xml.elements {
      if element.name == "measure" {
        for part_element in element.elements {
          let part_id = part_element
            .attributes
            .iter()
            .find(|(key, _)| key == "id")
            .ok_or("Missing \"id\" attribute in <measure> element")?
            .1
            .clone();
          parts
            .entry(part_id)
            .or_insert_with(|| XmlElement {
              name: part_element.name,
              attributes: part_element.attributes,
              elements: Vec::new(),
              text: part_element.text,
            })
            .elements
            .push(XmlElement {
              name: element.name.clone(),
              attributes: element.attributes.clone(),
              elements: part_element.elements,
              text: element.text.clone(),
            });
        }
      } else {
        converted_xml.elements.push(element);
      }
    }
    parts.into_values().for_each(|part| converted_xml.elements.push(part));
    Ok(converted_xml)
  } else {
    Err(String::from(
      "Root element in a MusicXML file must be either <score-partwise> or <score-timewise>",
    ))
  }
}

/// Parses a MusicXML string into a MusicXML element.
///
/// This function can be used to parse any MusicXML datatype or element from a string. It is not required that the
/// element being parsed be a top-level element such as `<score-partwise>` or `<score-timewise>`.
///
/// # Errors
///
/// If the string cannot be parsed into a valid MusicXML element, an error message will be returned.
pub fn parse_from_xml_str<T: ElementDeserializer>(str: &str) -> Result<T, String> {
  let xml = xml_parser::parse_from_string(str)?;
  T::deserialize(&xml)
}

/// Parses a MusicXML element into a MusicXML string.
///
/// This function can be used to convert any MusicXML datatype or element into a string. It is not required that the
/// element being parsed be a top-level element such as `<score-partwise>` or `<score-timewise>`.
pub fn parse_to_xml_str<T: ElementSerializer>(data: &T, pretty_print: bool) -> String {
  let xml = T::serialize(data);
  xml_parser::parse_to_string(&xml, if pretty_print { 0 } else { -1 })
}

/// Parses the contents of the specified MusicXML file into a [ScorePartwise] element.
///
/// The specified file can be either a `.musicxml` file or a compressed `.mxl` file.
///
/// # Errors
///
/// If the file does not exist, cannot be read, or is not a valid MusicXML file, an
/// error message will be returned.
pub fn parse_score_partwise_from_file(path: &str) -> Result<ScorePartwise, String> {
  let contents = get_musicxml_contents_from_file(path)?;
  let xml = xml_parser::parse_from_string(&contents)?;
  convert_xml_timewise_to_partwise(xml).and_then(|xml| ScorePartwise::deserialize(&xml))
}

/// Parses the contents of the specified MusicXML file into a [ScoreTimewise] element.
///
/// The specified file can be either a `.musicxml` file or a compressed `.mxl` file.
///
/// # Errors
///
/// If the file does not exist, cannot be read, or is not a valid MusicXML file, an
/// error message will be returned.
pub fn parse_score_timewise_from_file(path: &str) -> Result<ScoreTimewise, String> {
  let contents = get_musicxml_contents_from_file(path)?;
  let xml = xml_parser::parse_from_string(&contents)?;
  convert_xml_partwise_to_timewise(xml).and_then(|xml| ScoreTimewise::deserialize(&xml))
}

/// Parses the contents of the specified MusicXML data into a [ScorePartwise] element.
///
/// The specified data should have been read directly from either a `.musicxml` file or a compressed `.mxl` file.
///
/// # Errors
///
/// If the data cannot be parsed or does not represent valid MusicXML contents, an error message will be returned.
pub fn parse_score_partwise_from_data(data: Vec<u8>) -> Result<ScorePartwise, String> {
  let contents = get_musicxml_contents(data)?;
  let xml = xml_parser::parse_from_string(&contents)?;
  convert_xml_timewise_to_partwise(xml).and_then(|xml| ScorePartwise::deserialize(&xml))
}

/// Parses the contents of the specified MusicXML data into a [ScoreTimewise] element.
///
/// The specified data should have been read directly from either a `.musicxml` file or a compressed `.mxl` file.
///
/// # Errors
///
/// If the data cannot be parsed or does not represent valid MusicXML contents, an error message will be returned.
pub fn parse_score_timewise_from_data(data: Vec<u8>) -> Result<ScoreTimewise, String> {
  let contents = get_musicxml_contents(data)?;
  let xml = xml_parser::parse_from_string(&contents)?;
  convert_xml_partwise_to_timewise(xml).and_then(|xml| ScoreTimewise::deserialize(&xml))
}

/// Writes the contents of the specified [ScorePartwise] element into a MusicXML file.
///
/// If the `compressed` parameter is set to `true`, the MusicXML file will be written as a compressed `.mxl` file.
/// If the `write_timewise` parameter is set to `true`, the MusicXML file will be converted into a timewise format and
/// written as a `<score-timewise>` element.
///
/// The `pretty_print` parameter specifies whether the MusicXML file should be written with indentation and newlines.
///
/// # Errors
///
/// If the file cannot be written or the data cannot be serialized into a valid MusicXML format, an error message
/// will be returned.
pub fn parse_score_partwise_to_file(
  path: &str,
  score: &ScorePartwise,
  compressed: bool,
  pretty_print: bool,
  write_timewise: bool,
) -> Result<(), String> {
  let xml = ScorePartwise::serialize(score);
  if write_timewise {
    convert_xml_partwise_to_timewise(xml)
      .and_then(|xml| write_musicxml_contents_to_file(path, &xml, compressed, pretty_print))
  } else {
    write_musicxml_contents_to_file(path, &xml, compressed, pretty_print)
  }
}

/// Writes the contents of the specified [ScoreTimewise] element into a MusicXML file.
///
/// If the `compressed` parameter is set to `true`, the MusicXML file will be written as a compressed `.mxl` file.
/// If the `write_partwise` parameter is set to `true`, the MusicXML file will be converted into a partwise format and
/// written as a `<score-partwise>` element.
///
/// The `pretty_print` parameter specifies whether the MusicXML file should be written with indentation and newlines.
///
/// # Errors
///
/// If the file cannot be written or the data cannot be serialized into a valid MusicXML format, an error message
/// will be returned.
pub fn parse_score_timewise_to_file(
  path: &str,
  score: &ScoreTimewise,
  compressed: bool,
  pretty_print: bool,
  write_partwise: bool,
) -> Result<(), String> {
  let xml = ScoreTimewise::serialize(score);
  if write_partwise {
    convert_xml_timewise_to_partwise(xml)
      .and_then(|xml| write_musicxml_contents_to_file(path, &xml, compressed, pretty_print))
  } else {
    write_musicxml_contents_to_file(path, &xml, compressed, pretty_print)
  }
}

/// Writes the contents of the specified [ScorePartwise] element into a MusicXML data buffer.
///
/// If the `compressed` parameter is set to `true`, the MusicXML contents will be written as compressed `.mxl` data.
/// If the `write_timewise` parameter is set to `true`, the MusicXML contents will be converted into a timewise
/// format and written as a `<score-timewise>` element.
///
/// The `pretty_print` parameter specifies whether the MusicXML contents should be written with indentation
/// and newlines.
///
/// # Errors
///
/// If the data cannot be serialized into a valid MusicXML format, an error message will be returned.
pub fn parse_score_partwise_to_data(
  score: &ScorePartwise,
  compressed: bool,
  pretty_print: bool,
  write_timewise: bool,
) -> Result<Vec<u8>, String> {
  let xml = ScorePartwise::serialize(score);
  if write_timewise {
    convert_xml_partwise_to_timewise(xml).map(|xml| write_musicxml_contents(&xml, compressed, pretty_print))
  } else {
    Ok(write_musicxml_contents(&xml, compressed, pretty_print))
  }
}

/// Writes the contents of the specified [ScoreTimewise] element into a MusicXML data buffer.
///
/// If the `compressed` parameter is set to `true`, the MusicXML contents will be written as compressed `.mxl` data.
/// If the `write_partwise` parameter is set to `true`, the MusicXML contents will be converted into a partwise
/// format and written as a `<score-partwise>` element.
///
/// The `pretty_print` parameter specifies whether the MusicXML contents should be written with indentation
/// and newlines.
///
/// # Errors
///
/// If the data cannot be serialized into a valid MusicXML format, an error message will be returned.
pub fn parse_score_timewise_to_data(
  score: &ScoreTimewise,
  compressed: bool,
  pretty_print: bool,
  write_partwise: bool,
) -> Result<Vec<u8>, String> {
  let xml = ScoreTimewise::serialize(score);
  if write_partwise {
    convert_xml_timewise_to_partwise(xml).map(|xml| write_musicxml_contents(&xml, compressed, pretty_print))
  } else {
    Ok(write_musicxml_contents(&xml, compressed, pretty_print))
  }
}
