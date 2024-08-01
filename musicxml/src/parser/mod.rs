use crate::elements::{ScorePartwise, ScoreTimewise};
use alloc::{collections::BTreeMap, string::String, vec::Vec};
use musicxml_internal::{ElementDeserializer, ElementSerializer, XmlElement};

#[cfg(feature = "std")]
extern crate std;

#[cfg(feature = "std")]
use {
  alloc::string::ToString,
  std::io::{Read, Write},
};

mod xml_parser;

#[cfg(feature = "std")]
fn is_mxl_file(path: &str) -> bool {
  let mut buffer: [u8; 4] = [0; 4];
  if let Ok(mut file) = std::fs::OpenOptions::new().read(true).open(path) {
    file.read_exact(&mut buffer).is_ok() && (buffer == [0x50, 0x4b, 0x03, 0x04])
  } else {
    false
  }
}

#[cfg(feature = "std")]
fn get_musicxml_contents_from_file(path: &str) -> Result<String, String> {
  let mut contents = String::new();
  if is_mxl_file(path) {
    let mut xml_path: Option<String> = None;
    let mut archive =
      zip::ZipArchive::new(std::fs::File::open(path).map_err(|e| e.to_string())?).map_err(|e| e.to_string())?;
    for i in 0..archive.len() {
      let mut file = archive.by_index(i).map_err(|e| e.to_string())?;
      if file.name() == "META-INF/container.xml" {
        let mut buffer = String::new();
        file.read_to_string(&mut buffer).map_err(|e| e.to_string())?;
        let container = xml_parser::parse_from_string(&buffer)?;
        xml_path = container
          .elements
          .iter()
          .find(|&el| el.name == "rootfiles")
          .and_then(|el| el.elements.iter().find(|&el| el.name == "rootfile"))
          .and_then(|el| el.attributes.iter().find(|&attr| attr.0 == "full-path"))
          .and_then(|attr| Some(attr.1.clone()));
      }
    }
    if let Some(full_path) = &xml_path {
      let mut file = archive.by_name(full_path.as_str()).map_err(|e| e.to_string())?;
      file.read_to_string(&mut contents).map_err(|e| e.to_string())?;
    } else {
      Err(String::from("Cannot find MusicXML file in compressed archive"))?;
    }
  } else {
    let mut file = std::fs::OpenOptions::new()
      .read(true)
      .open(path)
      .map_err(|e| e.to_string())?;
    file.read_to_string(&mut contents).map_err(|e| e.to_string())?;
  }
  Ok(contents)
}

#[cfg(not(feature = "std"))]
fn get_musicxml_contents_from_file(_path: &str) -> Result<String, String> {
  Err(String::from(
    "Reading MusicXML files is not supported in a 'no_std' environment",
  ))
}

#[cfg(feature = "std")]
fn write_musicxml_file<T: Write>(file: &mut T, xml: &XmlElement, pretty_print: bool) -> Result<(), String> {
  file
    .write_all(b"<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n")
    .map_err(|e| e.to_string())?;
  if xml.name == "score-partwise" {
    file.write_all(b"<!DOCTYPE score-partwise PUBLIC \"-//Recordare//DTD MusicXML 3.0 Partwise//EN\" \"http://www.musicxml.org/dtds/partwise.dtd\">\n").map_err(|e| e.to_string())?;
  } else if xml.name == "score-timewise" {
    file.write_all(b"<!DOCTYPE score-timewise PUBLIC \"-//Recordare//DTD MusicXML 3.0 Timewise//EN\" \"http://www.musicxml.org/dtds/timewise.dtd\">\n").map_err(|e| e.to_string())?;
  }
  file
    .write_all(xml_parser::parse_to_string(xml, if pretty_print { 0 } else { -1 }).as_ref())
    .map_err(|e| e.to_string())
}

#[cfg(feature = "std")]
fn write_musicxml_to_file(path: &str, xml: &XmlElement, compressed: bool, pretty_print: bool) -> Result<(), String> {
  if compressed {
    let options = zip::write::SimpleFileOptions::default()
      .compression_method(zip::CompressionMethod::Deflated)
      .compression_level(Some(9));
    let mut archive = zip::ZipWriter::new(std::fs::File::create(path).map_err(|e| e.to_string())?);
    archive
      .start_file("META-INF/container.xml", options)
      .map_err(|e| e.to_string())?;
    archive.write_all(b"<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<container>\n  <rootfiles>\n    <rootfile full-path=\"score.musicxml\" media-type=\"application/vnd.recordare.musicxml+xml\"/>\n  </rootfiles>\n</container>")
      .map_err(|e| e.to_string())?;
    archive
      .start_file("score.musicxml", options)
      .map_err(|e| e.to_string())?;
    write_musicxml_file(&mut archive, xml, pretty_print)?;
    archive.finish().map_err(|e| e.to_string())?;
    Ok(())
  } else {
    let mut file = std::fs::OpenOptions::new()
      .write(true)
      .create(true)
      .open(path)
      .map_err(|e| e.to_string())?;
    write_musicxml_file(&mut file, xml, pretty_print)
  }
}

#[cfg(not(feature = "std"))]
fn write_musicxml_to_file(
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
            .unwrap()
            .1
            .parse()
            .unwrap();
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
            .unwrap()
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
pub fn parse_score_partwise_from_file(path: &str) -> Result<ScorePartwise, String> {
  let contents = get_musicxml_contents_from_file(path)?;
  let xml = xml_parser::parse_from_string(&contents)?;
  convert_xml_timewise_to_partwise(xml).and_then(|xml| ScorePartwise::deserialize(&xml))
}

/// Parses the contents of the specified MusicXML file into a [ScoreTimewise] element.
///
/// The specified file can be either a `.musicxml` file or a compressed `.mxl` file.
pub fn parse_score_timewise_from_file(path: &str) -> Result<ScoreTimewise, String> {
  let contents = get_musicxml_contents_from_file(path)?;
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
pub fn parse_score_partwise_to_file(
  path: &str,
  score: &ScorePartwise,
  compressed: bool,
  pretty_print: bool,
  write_timewise: bool,
) -> Result<(), String> {
  let xml = ScorePartwise::serialize(score);
  if write_timewise {
    convert_xml_partwise_to_timewise(xml).and_then(|xml| write_musicxml_to_file(path, &xml, compressed, pretty_print))
  } else {
    write_musicxml_to_file(path, &xml, compressed, pretty_print)
  }
}

/// Writes the contents of the specified [ScoreTimewise] element into a MusicXML file.
///
/// If the `compressed` parameter is set to `true`, the MusicXML file will be written as a compressed `.mxl` file.
/// If the `write_partwise` parameter is set to `true`, the MusicXML file will be converted into a partwise format and
/// written as a `<score-partwise>` element.
///
/// The `pretty_print` parameter specifies whether the MusicXML file should be written with indentation and newlines.
pub fn parse_score_timewise_to_file(
  path: &str,
  score: &ScoreTimewise,
  compressed: bool,
  pretty_print: bool,
  write_partwise: bool,
) -> Result<(), String> {
  let xml = ScoreTimewise::serialize(score);
  if write_partwise {
    convert_xml_timewise_to_partwise(xml).and_then(|xml| write_musicxml_to_file(path, &xml, compressed, pretty_print))
  } else {
    write_musicxml_to_file(path, &xml, compressed, pretty_print)
  }
}
