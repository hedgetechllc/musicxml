#![allow(dead_code)]

use alloc::{collections::BTreeMap, vec::Vec};
use core2::io::Read;
use libflate::deflate::{Decoder, Encoder};

const DEFLATE_METHOD: u16 = 8;
const LOCAL_FILE_HEADER_LEN: usize = core::mem::size_of::<LocalFileHeader>();
const CENTRAL_FILE_HEADER_LEN: usize = core::mem::size_of::<CentralFileHeader>();
const CENTRAL_DIR_END_LEN: usize = core::mem::size_of::<CentralDirEnd>();

#[repr(packed)]
pub(crate) struct LocalFileHeader {
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
    unsafe { (ptr.as_ptr() as *const Self).as_ref().unwrap_unchecked() }
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
    unsafe { (bytes.as_ptr() as *const Self).as_ref().unwrap_unchecked() }
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
    unsafe { (bytes.as_ptr() as *const Self).as_ref().unwrap_unchecked() }
  }
}

pub(crate) struct LocalFile {
  pub file_name: String,
  pub compressed_size: u64,
  pub uncompressed_size: u64,
  pub data: Vec<u8>,
}

impl LocalFile {
  pub fn new(file_name: String, data: &[u8], details: &LocalFileHeader) -> Self {
    let mut decoded_data = Vec::new();
    let mut decoder = Decoder::new(&data[details.len()..(details.len() + details.compressed_size as usize)]);
    let _ = decoder.read_to_end(&mut decoded_data);
    Self {
      file_name,
      compressed_size: details.compressed_size as u64,
      uncompressed_size: details.uncompressed_size as u64,
      data: decoded_data,
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
              unsafe {
                core::ptr::copy(self.buffer.as_ptr().add(i), self.buffer.as_mut_ptr(), len);
              }
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
                  unsafe {
                    core::ptr::copy(
                      self.buffer.as_ptr().add(4),
                      self.buffer.as_mut_ptr(),
                      self.buffer_len - 4,
                    );
                  }
                  self.buffer_len -= 4;
                  entry_found = false;
                }
              }
            }
          } else if bytes_read > 2 {
            unsafe {
              core::ptr::copy(self.buffer.as_ptr().add(bytes_read - 3), self.buffer.as_mut_ptr(), 3);
            }
            self.buffer_len = 3;
          } else {
            return None;
          }
        }
      }
    }

    // Read the file name, header, and compression method
    let (file_name, file_header, compression_method, entry_length) = {
      let file_info = CentralFileHeader::from_bytes(&self.buffer);
      let mut file_name_buffer = vec![0; file_info.file_name_length as usize];
      unsafe {
        core::ptr::copy(
          self.buffer.as_ptr().add(CENTRAL_FILE_HEADER_LEN),
          file_name_buffer.as_mut_ptr(),
          file_name_buffer.len(),
        );
        (
          String::from_utf8_unchecked(file_name_buffer),
          &self.contents.content[file_info.relative_offset_of_local_header as usize..],
          file_info.compression_method,
          file_info.len(),
        )
      }
    };

    // Move remainder of the contents to beginning of the buffer for next iteration
    unsafe {
      core::ptr::copy(
        self.buffer.as_ptr().add(entry_length),
        self.buffer.as_mut_ptr(),
        self.buffer_len - entry_length,
      );
    }
    self.buffer_len -= entry_length;

    // Decompress the file if its method is DEFLATE
    if compression_method != DEFLATE_METHOD {
      self.next()
    } else {
      Some(LocalFile::new(
        file_name,
        file_header,
        LocalFileHeader::from_bytes(file_header),
      ))
    }
  }
}

pub(crate) struct ZipArchive {
  file_map: BTreeMap<String, LocalFile>,
}

impl ZipArchive {
  pub fn new(zip_data: &mut ZipData) -> Self {
    let mut file_map = BTreeMap::new();
    for file in ZipParser::new(zip_data) {
      file_map.insert(file.file_name.clone(), file);
    }
    Self { file_map }
  }

  pub fn get_file(&self, file_name: &str) -> Option<&LocalFile> {
    self.file_map.get(file_name)
  }

  pub fn iter(&self) -> impl Iterator<Item = &LocalFile> {
    self.file_map.values()
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
    for item in ZipArchive::new(&mut zip_data).iter() {
      println!("File: {}, Size: {}", item.file_name, item.uncompressed_size);
      println!("Content: {}", core::str::from_utf8(item.data.as_slice()).unwrap());
    }
  }
}
