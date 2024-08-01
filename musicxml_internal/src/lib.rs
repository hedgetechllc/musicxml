#![no_std]

extern crate alloc;

use alloc::string::{String, ToString};
use alloc::vec::Vec;

#[derive(Debug, Default, PartialEq, Eq)]
pub struct XmlElement {
  pub name: String,
  pub attributes: Vec<(String, String)>,
  pub elements: Vec<XmlElement>,
  pub text: String,
}

pub trait DatatypeDeserializer: Sized {
  fn deserialize(value: &str) -> Result<Self, String>;
}

pub trait AttributeDeserializer: Sized {
  fn deserialize(attributes: &Vec<(String, String)>) -> Result<Self, String>;
}

pub trait ContentDeserializer: Sized {
  fn deserialize(elements: &Vec<XmlElement>) -> Result<Self, String>;
}

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
