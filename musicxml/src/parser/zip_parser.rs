#![allow(dead_code)]

use alloc::{collections::BTreeMap, vec::Vec};
use crc32fast;
use miniz_oxide::deflate::compress_to_vec;
use miniz_oxide::inflate::decompress_to_vec;
use musicxml_internal::bytes_to_string;

#[cfg(feature = "std")]
use std::time::{SystemTime, UNIX_EPOCH};

#[cfg(not(feature = "std"))]
use alloc::string::{String, ToString};

const DEFLATE_METHOD_CODE: u16 = 8;
const LOCAL_FILE_HEADER_LEN: usize = core::mem::size_of::<LocalFileHeader>();
const CENTRAL_FILE_HEADER_LEN: usize = core::mem::size_of::<CentralFileHeader>();
const CENTRAL_DIR_END_LEN: usize = core::mem::size_of::<CentralDirEnd>();

#[repr(C, packed)]
struct LocalFileHeader {
  signature: u32,
  version_needed_to_extract: u16,
  general_purpose_bit_flag: u16,
  compression_method: u16,
  last_mod_file_time: u16,
  last_mod_file_date: u16,
  crc32: u32,
  compressed_size: u32,
  uncompressed_size: u32,
  file_name_length: u16,
  extra_field_length: u16,
}

impl LocalFileHeader {
  pub fn len(&self) -> usize {
    LOCAL_FILE_HEADER_LEN + self.file_name_length as usize + self.extra_field_length as usize
  }

  pub fn from_bytes(ptr: &[u8]) -> &Self {
    unsafe { ptr.as_ptr().cast::<Self>().as_ref().unwrap_unchecked() }
  }
}

#[repr(C, packed)]
struct CentralFileHeader {
  signature: u32,
  version_made_by: u16,
  version_needed_to_extract: u16,
  general_purpose_bit_flag: u16,
  compression_method: u16,
  last_mod_file_time: u16,
  last_mod_file_date: u16,
  crc32: u32,
  compressed_size: u32,
  uncompressed_size: u32,
  file_name_length: u16,
  extra_field_length: u16,
  file_comment_length: u16,
  disk_number_start: u16,
  internal_file_attributes: u16,
  external_file_attributes: u32,
  relative_offset_of_local_header: u32,
}

impl CentralFileHeader {
  pub fn len(&self) -> usize {
    CENTRAL_FILE_HEADER_LEN
      + self.file_name_length as usize
      + self.extra_field_length as usize
      + self.file_comment_length as usize
  }

  pub fn from_bytes(bytes: &[u8]) -> &Self {
    unsafe { bytes.as_ptr().cast::<Self>().as_ref().unwrap_unchecked() }
  }
}

#[repr(C, packed)]
struct CentralDirEnd {
  signature: u32,
  number_of_disk: u16,
  number_of_start_central_directory_disk: u16,
  total_entries_this_disk: u16,
  total_entries_all_disk: u16,
  size_of_the_central_directory: u32,
  central_directory_offset: u32,
  zip_file_comment_length: u16,
}

impl CentralDirEnd {
  pub fn from_bytes(bytes: &[u8]) -> &Self {
    unsafe { bytes.as_ptr().cast::<Self>().as_ref().unwrap_unchecked() }
  }
}

struct LocalFile {
  pub file_name: String,
  pub relative_offset: usize,
  pub compressed_size: usize,
  pub uncompressed_size: usize,
}

impl LocalFile {
  pub fn new(file_name: String, file_offset: usize, details: &LocalFileHeader) -> Self {
    Self {
      file_name,
      relative_offset: file_offset,
      compressed_size: details.compressed_size as usize,
      uncompressed_size: details.uncompressed_size as usize,
    }
  }
}

pub(crate) struct ZipData {
  index: usize,
  pub content: Vec<u8>,
}

impl ZipData {
  pub fn new() -> Self {
    Self {
      index: 0,
      content: Vec::new(),
    }
  }

  fn read(&mut self, buf: &mut [u8]) -> usize {
    let to_read = core::cmp::min(self.content.len() - self.index, buf.len());
    buf[..to_read].copy_from_slice(&self.content[self.index..self.index + to_read]);
    self.index += to_read;
    to_read
  }

  fn read_exact(&mut self, buf: &mut [u8]) -> Option<usize> {
    match self.read(buf) {
      bytes_read if bytes_read == buf.len() => Some(bytes_read),
      _ => None,
    }
  }
}

struct ZipParser<'a> {
  contents: &'a mut ZipData,
  buffer: [u8; 128],
  buffer_len: usize,
}

impl<'a> ZipParser<'a> {
  pub fn new(contents: &'a mut ZipData) -> Self {
    Self {
      contents,
      buffer: [0; 128],
      buffer_len: 0,
    }
  }
}

impl<'a> Iterator for ZipParser<'a> {
  type Item = LocalFile;

  fn next(&mut self) -> Option<Self::Item> {
    // Search for a Central Directory File Entry
    let mut entry_found = false;
    while !entry_found {
      match self.contents.read(&mut self.buffer[self.buffer_len..]) {
        bytes_read if self.buffer_len + bytes_read < 4 => return None,
        bytes_read => {
          for i in 0..(self.buffer_len + bytes_read - 4) {
            if self.buffer[i..i + 4] == [0x50, 0x4b, 0x01, 0x02] {
              let len = self.buffer_len + bytes_read - i;
              self.buffer.copy_within(i..(len + i), 0);
              self.buffer_len = len;
              entry_found = true;
              break;
            }
          }
          if entry_found {
            match self.contents.read(&mut self.buffer[self.buffer_len..]) {
              bytes_read if self.buffer_len + bytes_read < CENTRAL_FILE_HEADER_LEN => return None,
              bytes_read => {
                self.buffer_len += bytes_read;
                if self.buffer[28] < 8 || self.buffer[29] > 0 {
                  self.buffer.copy_within(4..self.buffer_len, 0);
                  self.buffer_len -= 4;
                  entry_found = false;
                }
              }
            }
          } else if bytes_read > 2 {
            self.buffer.copy_within((bytes_read - 3).., 0);
            self.buffer_len = 3;
          } else {
            return None;
          }
        }
      }
    }

    // Read the file name, header, and compression method
    let (file_name, file_header, file_offset, compression_method, entry_length) = {
      let file_info = CentralFileHeader::from_bytes(&self.buffer);
      (
        self.buffer[CENTRAL_FILE_HEADER_LEN..(CENTRAL_FILE_HEADER_LEN + file_info.file_name_length as usize)]
          .iter()
          .map(|&x| x as char)
          .collect::<String>(),
        &self.contents.content[file_info.relative_offset_of_local_header as usize..],
        file_info.relative_offset_of_local_header as usize,
        file_info.compression_method,
        file_info.len(),
      )
    };

    // Move remainder of the contents to beginning of the buffer for next iteration
    self.buffer.copy_within(entry_length..self.buffer_len, 0);
    self.buffer_len -= entry_length;

    // Decompress the file if its compression method is DEFLATE
    if compression_method == DEFLATE_METHOD_CODE {
      let header = LocalFileHeader::from_bytes(file_header);
      Some(LocalFile::new(file_name, file_offset + header.len(), header))
    } else {
      self.next()
    }
  }
}

pub(crate) struct ZipArchive<'a> {
  zip_data: &'a mut ZipData,
  file_map: BTreeMap<String, LocalFile>,
}

impl<'a> ZipArchive<'a> {
  pub fn new(zip_data: &'a mut ZipData) -> Self {
    let mut file_map = BTreeMap::new();
    for file in ZipParser::new(zip_data) {
      file_map.insert(file.file_name.clone(), file);
    }
    Self { zip_data, file_map }
  }

  pub fn read_file_to_string(&self, file_name: &str) -> Result<String, String> {
    let file = self
      .file_map
      .get(file_name)
      .ok_or(format!("File \"{file_name}\" not found within compressed archive"))?;
    let decoded_data =
      decompress_to_vec(&self.zip_data.content[file.relative_offset..(file.relative_offset + file.compressed_size)])
        .map_err(|e| e.to_string())?;
    bytes_to_string(&decoded_data).map_err(|e| e.to_string())
  }

  pub fn iter(&self) -> impl Iterator<Item = &String> {
    self.file_map.keys()
  }
}

pub struct ZipArchiver {
  content: Vec<u8>,
  files: Vec<(String, LocalFileHeader, usize)>,
  open_file: Option<(String, LocalFileHeader, Vec<u8>)>,
  current_offset: usize,
}

#[allow(clippy::cast_possible_truncation)]
impl ZipArchiver {
  pub fn new() -> Self {
    Self {
      content: Vec::new(),
      files: Vec::new(),
      open_file: None,
      current_offset: 0,
    }
  }

  #[cfg(feature = "std")]
  fn current_datetime() -> (u16, u16, u16, u16, u16, u16) {
    let now = SystemTime::now()
      .duration_since(UNIX_EPOCH)
      .unwrap_or_default()
      .as_secs();
    let year = 1970 + now / 31_556_952;
    let month = 1 + now % 31_556_952 / 2_629_746;
    let day = now % 2_629_746 / 86_400;
    let hours = now % 86_400 / 3600;
    let minutes = now % 3600 / 60;
    let seconds = now % 60;
    (
      year as u16,
      month as u16,
      day as u16,
      hours as u16,
      minutes as u16,
      seconds as u16,
    )
  }

  #[cfg(not(feature = "std"))]
  fn current_datetime() -> (u16, u16, u16, u16, u16, u16) {
    (2024, 1, 1, 0, 0, 0)
  }

  fn dostime(hour: u16, minute: u16, second: u16) -> u16 {
    let time = ((hour & 0b1_1111) << 11) | ((minute & 0b11_1111) << 5) | ((second / 2) & 0b1_1111);
    (time >> 8) | ((time & 0x00FF) << 8)
  }

  fn dosdate(year: u16, month: u16, day: u16) -> u16 {
    let date = ((year - 1980) << 9) | ((month & 0b1111) << 5) | (day & 0b1_1111);
    (date >> 8) | ((date & 0x00FF) << 8)
  }

  fn finish_open_file(&mut self) {
    if let Some((file_name, mut header, mut data)) = self.open_file.take() {
      header.crc32 = crc32fast::hash(data.as_slice());
      header.uncompressed_size = data.len() as u32;
      data = compress_to_vec(data.as_slice(), 10);
      header.compressed_size = data.len() as u32;
      self.content.extend_from_slice(unsafe {
        core::slice::from_raw_parts(core::ptr::from_ref(&header).cast::<u8>(), LOCAL_FILE_HEADER_LEN)
      });
      self.content.extend_from_slice(file_name.as_bytes());
      self.content.append(&mut data);
      self.files.push((file_name, header, self.current_offset));
      self.current_offset = self.content.len();
    }
  }

  pub fn start_file(&mut self, file_name: &str) {
    self.finish_open_file();
    let file_name_bytes = file_name.as_bytes();
    let file_name_len = file_name_bytes.len();
    let (year, month, day, hour, minute, second) = Self::current_datetime();
    let header = LocalFileHeader {
      signature: 0x0403_4b50,
      version_needed_to_extract: 20,
      general_purpose_bit_flag: 0b1000_0000_0000,
      compression_method: DEFLATE_METHOD_CODE,
      last_mod_file_time: Self::dostime(hour, minute, second),
      last_mod_file_date: Self::dosdate(year, month, day),
      crc32: 0,
      compressed_size: 0,
      uncompressed_size: 0,
      file_name_length: file_name_len as u16,
      extra_field_length: 0,
    };
    self.open_file = Some((String::from(file_name), header, Vec::new()));
  }

  pub fn write_data(&mut self, data: &[u8]) {
    if let Some((_, _, file_data)) = self.open_file.as_mut() {
      file_data.extend_from_slice(data);
    }
  }

  pub fn finish(&mut self) -> Vec<u8> {
    self.finish_open_file();
    for (file_name, header, file_offset) in &self.files {
      let header = CentralFileHeader {
        signature: 0x0201_4b50,
        version_made_by: header.version_needed_to_extract,
        version_needed_to_extract: header.version_needed_to_extract,
        general_purpose_bit_flag: header.general_purpose_bit_flag,
        compression_method: header.compression_method,
        last_mod_file_time: header.last_mod_file_time,
        last_mod_file_date: header.last_mod_file_date,
        crc32: header.crc32,
        compressed_size: header.compressed_size,
        uncompressed_size: header.uncompressed_size,
        file_name_length: header.file_name_length,
        extra_field_length: 0,
        file_comment_length: 0,
        disk_number_start: 0,
        internal_file_attributes: 0,
        external_file_attributes: 0,
        relative_offset_of_local_header: *file_offset as u32,
      };
      let header_bytes =
        unsafe { core::slice::from_raw_parts(core::ptr::from_ref(&header).cast::<u8>(), CENTRAL_FILE_HEADER_LEN) };
      self.content.extend_from_slice(header_bytes);
      self.content.extend_from_slice(file_name.as_bytes());
    }
    let central_dir_end = CentralDirEnd {
      signature: 0x0605_4b50,
      number_of_disk: 0,
      number_of_start_central_directory_disk: 0,
      total_entries_this_disk: self.files.len() as u16,
      total_entries_all_disk: self.files.len() as u16,
      size_of_the_central_directory: (self.content.len() - self.current_offset) as u32,
      central_directory_offset: self.current_offset as u32,
      zip_file_comment_length: 0,
    };
    let central_dir_end_bytes =
      unsafe { core::slice::from_raw_parts(core::ptr::from_ref(&central_dir_end).cast::<u8>(), CENTRAL_DIR_END_LEN) };
    self.content.extend_from_slice(central_dir_end_bytes);
    self.content.clone()
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use std::io::Read;

  #[test]
  fn test_zip_parser() {
    let mut zip_data = ZipData::new();
    std::fs::File::open("tests/Grande Valse Brillante.mxl")
      .unwrap()
      .read_to_end(&mut zip_data.content)
      .unwrap_or(0);
    let zip_archive = ZipArchive::new(&mut zip_data);
    for item in zip_archive.iter() {
      println!("File: {}", item);
      println!("Content: {}", zip_archive.read_file_to_string(item).unwrap());
    }
  }

  #[test]
  fn test_zip_creator() {
    let mut archiver = ZipArchiver::new();
    archiver.start_file("META-INF/container.xml");
    archiver.write_data(b"<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<container>\n  <rootfiles>\n    <rootfile full-path=\"score.musicxml\" media-type=\"application/vnd.recordare.musicxml+xml\"/>\n  </rootfiles>\n</container>");
    archiver.start_file("score.musicxml");
    archiver.write_data(b"Test Data");
    let zip_data = archiver.finish();
    assert_eq!(
      zip_data[0..10],
      [0x50, 0x4B, 0x03, 0x04, 0x14, 0x00, 0x00, 0x08, 0x08, 0x00]
    );
    assert_eq!(
      zip_data[14..196],
      [
        0x58, 0x11, 0xA4, 0x95, 0x86, 0x00, 0x00, 0x00, 0xBB, 0x00, 0x00, 0x00, 0x16, 0x00, 0x00, 0x00, 0x4D, 0x45,
        0x54, 0x41, 0x2D, 0x49, 0x4E, 0x46, 0x2F, 0x63, 0x6F, 0x6E, 0x74, 0x61, 0x69, 0x6E, 0x65, 0x72, 0x2E, 0x78,
        0x6D, 0x6C, 0x4D, 0x8E, 0x3B, 0x0E, 0x02, 0x21, 0x14, 0x45, 0xFB, 0x59, 0x05, 0x79, 0xAD, 0x01, 0xB4, 0xB3,
        0x80, 0x99, 0xCE, 0x15, 0xE8, 0x02, 0x08, 0xBC, 0xD1, 0x97, 0xF0, 0x0B, 0x30, 0x13, 0xDD, 0xBD, 0x8C, 0x8D,
        0x94, 0x27, 0x39, 0x37, 0xE7, 0xAA, 0xE5, 0x1D, 0x3C, 0xDB, 0xB1, 0x54, 0x4A, 0x51, 0xC3, 0x45, 0x9C, 0x81,
        0x61, 0xB4, 0xC9, 0x51, 0x7C, 0x6A, 0x78, 0xDC, 0x6F, 0xFC, 0x0A, 0xCB, 0x3C, 0x29, 0x9B, 0x62, 0x33, 0x14,
        0xB1, 0xCC, 0x13, 0x63, 0xAA, 0xA4, 0xD4, 0x56, 0xF2, 0x58, 0x0F, 0x1A, 0x98, 0xAD, 0x9B, 0xF7, 0x3C, 0x9B,
        0xF6, 0xD2, 0x50, 0x6D, 0x2A, 0x28, 0xC2, 0x56, 0xC9, 0xF6, 0x04, 0xB0, 0x80, 0x8E, 0x0C, 0x6F, 0x9F, 0x8C,
        0x1A, 0x4C, 0xCE, 0x9E, 0xAC, 0x69, 0xBD, 0x29, 0xF7, 0xE8, 0x44, 0xC1, 0x2E, 0x3B, 0x33, 0xF8, 0xA7, 0x63,
        0x23, 0x7F, 0x31, 0x39, 0xD4, 0x94, 0xFC, 0x1F, 0xF9, 0x02, 0x50, 0x4B, 0x03, 0x04, 0x14, 0x00, 0x00, 0x08,
        0x08, 0x00
      ]
    );
    assert_eq!(
      zip_data[200..253],
      [
        0x2F, 0x83, 0xCB, 0xF1, 0x0B, 0x00, 0x00, 0x00, 0x09, 0x00, 0x00, 0x00, 0x0E, 0x00, 0x00, 0x00, 0x73, 0x63,
        0x6F, 0x72, 0x65, 0x2E, 0x6D, 0x75, 0x73, 0x69, 0x63, 0x78, 0x6D, 0x6C, 0x0B, 0x49, 0x2D, 0x2E, 0x51, 0x70,
        0x49, 0x2C, 0x49, 0x04, 0x00, 0x50, 0x4B, 0x01, 0x02, 0x14, 0x00, 0x14, 0x00, 0x00, 0x08, 0x08, 0x00
      ]
    );
    assert_eq!(
      zip_data[257..321],
      [
        0x58, 0x11, 0xA4, 0x95, 0x86, 0x00, 0x00, 0x00, 0xBB, 0x00, 0x00, 0x00, 0x16, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x4D, 0x45, 0x54, 0x41, 0x2D, 0x49,
        0x4E, 0x46, 0x2F, 0x63, 0x6F, 0x6E, 0x74, 0x61, 0x69, 0x6E, 0x65, 0x72, 0x2E, 0x78, 0x6D, 0x6C, 0x50, 0x4B,
        0x01, 0x02, 0x14, 0x00, 0x14, 0x00, 0x00, 0x08, 0x08, 0x00
      ]
    );
    assert_eq!(
      zip_data[325..],
      [
        0x2F, 0x83, 0xCB, 0xF1, 0x0B, 0x00, 0x00, 0x00, 0x09, 0x00, 0x00, 0x00, 0x0E, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xBA, 0x00, 0x00, 0x00, 0x73, 0x63, 0x6F, 0x72, 0x65, 0x2E,
        0x6D, 0x75, 0x73, 0x69, 0x63, 0x78, 0x6D, 0x6C, 0x50, 0x4B, 0x05, 0x06, 0x00, 0x00, 0x00, 0x00, 0x02, 0x00,
        0x02, 0x00, 0x80, 0x00, 0x00, 0x00, 0xF1, 0x00, 0x00, 0x00, 0x00, 0x00
      ]
    );
  }
}
