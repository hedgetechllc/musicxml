use musicxml::*;

fn main() {
  // Read a MusicXML file of one score type and write it as a different score type
  let output_path = "./target/Grande Valse Brillante.musicxml";
  let score =
    read_score_partwise("./musicxml/tests/Grande Valse Brillante.musicxml").expect("Failed to read input test file");
  println!(
    "Converting and writing to '{}'...{}",
    output_path,
    if write_partwise_score(output_path, &score, false, true).is_ok() {
      "SUCCESS"
    } else {
      "FAILED"
    }
  );
}
