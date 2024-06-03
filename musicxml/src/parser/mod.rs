use musicxml_internal::{ElementDeserializer, ElementSerializer};
use std::io::{Read, Write};

mod xml_parser;

fn is_mxl_file(path: &str) -> bool {
  let mut buffer: [u8; 4] = [0; 4];
  if let Ok(mut file) = std::fs::OpenOptions::new().read(true).open(path) {
    file.read_exact(&mut buffer).is_ok() && (buffer == [0x50, 0x4b, 0x03, 0x04])
  } else {
    false
  }
}

pub fn parse_to_xml_str<T: ElementSerializer>(data: &T, pretty_print: bool) -> String {
  let xml = T::serialize(data);
  xml_parser::write_xml_to_str(&xml, if pretty_print { 0 } else { -1 })
}

pub fn parse_to_xml_file<T: ElementSerializer>(
  path: &str,
  data: &T,
  compressed: bool,
  pretty_print: bool,
) -> Result<(), String> {
  let xml = T::serialize(data);
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
    archive
      .write_all(b"<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n")
      .map_err(|e| e.to_string())?;
    if xml.name == "score-partwise" {
      archive.write_all(b"<!DOCTYPE score-partwise PUBLIC \"-//Recordare//DTD MusicXML 3.0 Partwise//EN\" \"http://www.musicxml.org/dtds/partwise.dtd\">\n").map_err(|e| e.to_string())?;
    } else if xml.name == "score-timewise" {
      archive.write_all(b"<!DOCTYPE score-timewise PUBLIC \"-//Recordare//DTD MusicXML 3.0 Timewise//EN\" \"http://www.musicxml.org/dtds/timewise.dtd\">\n").map_err(|e| e.to_string())?;
    }
    archive
      .write_all(xml_parser::write_xml_to_str(&xml, if pretty_print { 0 } else { -1 }).as_ref())
      .map_err(|e| e.to_string())?;
    archive.finish().map_err(|e| e.to_string())?;
    Ok(())
  } else {
    xml_parser::write_xml_to_file(path, &xml, pretty_print)
  }
}

pub fn parse_from_xml_str<T: ElementDeserializer>(str: &str) -> Result<T, String> {
  let xml = xml_parser::parse_xml_from_str(str)?;
  T::deserialize(&xml)
}

pub fn parse_from_xml_file<T: ElementDeserializer>(path: &str) -> Result<T, String> {
  if is_mxl_file(path) {
    let mut xml_path: Option<String> = None;
    let mut archive =
      zip::ZipArchive::new(std::fs::File::open(path).map_err(|e| e.to_string())?).map_err(|e| e.to_string())?;
    for i in 0..archive.len() {
      let mut file = archive.by_index(i).map_err(|e| e.to_string())?;
      if file.name() == "META-INF/container.xml" {
        let mut buffer = String::new();
        file.read_to_string(&mut buffer).map_err(|e| e.to_string())?;
        let container = xml_parser::parse_xml_from_str(&buffer)?;
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
      let mut buffer = String::new();
      let mut file = archive.by_name(full_path.as_str()).map_err(|e| e.to_string())?;
      file.read_to_string(&mut buffer).map_err(|e| e.to_string())?;
      T::deserialize(&xml_parser::parse_xml_from_str(&buffer)?)
    } else {
      Err(String::from("Cannot find MusicXML file in compressed archive"))
    }
  } else {
    let xml = xml_parser::parse_xml_from_file(path)?;
    T::deserialize(&xml)
  }
}
