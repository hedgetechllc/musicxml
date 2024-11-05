#![no_std]

extern crate alloc;

use alloc::borrow::ToOwned;
use alloc::string::{String, ToString};
use alloc::vec::Vec;

pub fn bytes_to_string(bytes: &[u8]) -> Result<String, String> {
  String::from_utf8(bytes.to_owned()).or_else(|_| {
    let convert = if bytes[0] == 0xFF && bytes[1] == 0xFE {
      u16::from_le_bytes
    } else {
      u16::from_be_bytes
    };
    let u16_bytes: Vec<u16> = bytes
      .chunks_exact(2)
      .map(|bytes| convert([bytes[0], bytes[1]]))
      .collect();
    String::from_utf16(&u16_bytes).map_err(|_| String::from("Invalid UTF-8 or UTF-16 data in MusicXML content"))
  })
}

#[derive(Debug, Default, PartialEq, Eq)]
pub struct XmlElement {
  pub name: String,
  pub attributes: Vec<(String, String)>,
  pub elements: Vec<XmlElement>,
  pub text: String,
}

#[allow(clippy::missing_errors_doc)]
pub trait DatatypeDeserializer: Sized {
  fn deserialize(value: &str) -> Result<Self, String>;
}

#[allow(clippy::missing_errors_doc)]
pub trait AttributeDeserializer: Sized {
  fn deserialize(attributes: &[(String, String)]) -> Result<Self, String>;
}

#[allow(clippy::missing_errors_doc)]
pub trait ContentDeserializer: Sized {
  fn deserialize(elements: &[XmlElement]) -> Result<Self, String>;
}

#[allow(clippy::missing_errors_doc)]
pub trait ElementDeserializer: Sized {
  fn deserialize(element: &XmlElement) -> Result<Self, String>;
}

pub trait DatatypeSerializer {
  fn serialize(element: &Self) -> String;
}

pub trait AttributeSerializer {
  fn serialize(element: &Self) -> Vec<(String, String)>;
}

pub trait ContentSerializer {
  fn serialize(element: &Self) -> Vec<XmlElement>;
}

pub trait ElementSerializer {
  fn serialize(element: &Self) -> XmlElement;
}

impl DatatypeDeserializer for String {
  fn deserialize(value: &str) -> Result<Self, String> {
    Ok(String::from(value))
  }
}

impl DatatypeSerializer for String {
  fn serialize(element: &Self) -> String {
    element.clone()
  }
}

impl DatatypeDeserializer for i8 {
  fn deserialize(value: &str) -> Result<Self, String> {
    value.parse::<Self>().map_err(|err| err.to_string())
  }
}

impl DatatypeSerializer for i8 {
  fn serialize(element: &Self) -> String {
    element.to_string()
  }
}

impl DatatypeDeserializer for i16 {
  fn deserialize(value: &str) -> Result<Self, String> {
    value.parse::<Self>().map_err(|err| err.to_string())
  }
}

impl DatatypeSerializer for i16 {
  fn serialize(element: &Self) -> String {
    element.to_string()
  }
}
