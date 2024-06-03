use std::{fs::File, io::{BufReader, Read}};

use musicxml::*;

fn main() {
  // Read, parse, and rewrite a MusicXML file
  let output_path = "./musicxml/target/Grande Valse Brillante.musicxml";
  let score = read_score_partwise("./musicxml/tests/Grande Valse Brillante.musicxml").expect("Failed to read input test file");
  println!("Writing to '{}'...{}", output_path, if write_score_partwise(&score, output_path).is_ok() { "SUCCESS" } else { "FAILED" });

  // Verify that the contents of the original and rewritten files are identical
  println!("Checking if input and output files are identical...");
  let file1 = File::open("./musicxml/tests/Grande Valse Brillante.musicxml").expect("Input test file not found");
  let file2 = File::open(output_path).expect("Output file not found");
  if file1.metadata().unwrap().len() != file2.metadata().unwrap().len() {
    println!("FAILURE: Sizes are different");
  } else {
    let mut reader1 = BufReader::new(file1);
    let mut reader2 = BufReader::new(file2);
    let mut buffer1 = Vec::new();
    let mut buffer2 = Vec::new();
    reader1.read_to_end(&mut buffer1).expect("Read failed");
    reader2.read_to_end(&mut buffer2).expect("Read failed");
    if buffer1 != buffer2 {
      println!("FAILURE: Contents are different");
    } else {
      println!("SUCCESS");
    }
  }
}
