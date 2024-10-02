#![allow(dead_code)]

use alloc::{collections::BTreeMap, vec::Vec};
use core2::io::Read;
use libflate::deflate::Decoder;

const DEFLATE_METHOD_CODE: u16 = 8;
const LOCAL_FILE_HEADER_LEN: usize = core::mem::size_of::<LocalFileHeader>();
const CENTRAL_FILE_HEADER_LEN: usize = core::mem::size_of::<CentralFileHeader>();
const CENTRAL_DIR_END_LEN: usize = core::mem::size_of::<CentralDirEnd>();

#[repr(packed)]
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

#[repr(packed)]
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

#[repr(packed)]
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
    let mut decoded_data = Vec::new();
    let file = self
      .file_map
      .get(file_name)
      .ok_or(format!("File \"{file_name}\" not found within compressed archive"))?;
    let mut decoder =
      Decoder::new(&self.zip_data.content[file.relative_offset..(file.relative_offset + file.compressed_size)]);
    decoder.read_to_end(&mut decoded_data).unwrap_or(0);
    String::from_utf8(decoded_data).map_err(|e| e.to_string())
  }

  pub fn iter(&self) -> impl Iterator<Item = &String> {
    self.file_map.keys()
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
}
